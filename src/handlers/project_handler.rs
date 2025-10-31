use aws_sdk_s3::Client as S3Client;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode as AxumStatusCode,
};
use reqwest::{Client as HttpClient, StatusCode as HttpStatusCode};
use std::{
    borrow::ToOwned,
    collections::{HashMap, HashSet},
};

use serde::Deserialize;

use crate::{
    error::ApiError,
    models::{
        image::Image as ProjectImage,
        project::{
            Project,
            child::{floorplan::Floorplan, structure::BoundingBox},
        },
    },
    repositories::{
        floor_structure_repository::FloorStructureRecord, project_repository::ProjectRepository,
        room_structure_repository::RoomStructureRecord,
    },
    routes::project::dto::{
        FloorResponse, ProjectRenderingImageResponse, ProjectRenderingsResponse, ProjectResponse,
        RoomItemsResponse, RoomResponse,
    },
    state::AppState,
};

#[derive(Debug, Default, Deserialize)]
pub struct AreaRangeQuery {
    #[serde(rename = "areaFrom")]
    area_from: Option<i32>,
    #[serde(rename = "areaTo")]
    area_to: Option<i32>,
}

pub async fn get_project_by_id(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let repository = state.project_repository()?;
    let mut project = repository.get_by_id(&project_id).await?;
    populate_floorplans(&state.http_client, &state.cdn_base_url, &mut project).await?;
    ensure_default_cover_image(
        repository,
        &mut project,
        state.s3_client.as_ref(),
        state.s3_bucket.as_deref(),
        &state.cdn_base_url,
    )
    .await?;
    let response = ProjectResponse::try_from_project(&project).map_err(ApiError::internal)?;
    Ok(Json(response))
}

pub async fn get_project_renderings(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectRenderingsResponse>, ApiError> {
    let project_repository = state.project_repository()?;
    let image_repository = state.image_repository()?;

    let project = project_repository.get_by_id(&project_id).await?;
    let image_ids = project.image_ids.unwrap_or_default();
    if image_ids.is_empty() {
        return Ok(Json(ProjectRenderingsResponse::new(Vec::new())));
    }

    let images = image_repository.find_by_ids(&image_ids).await?;

    let mut image_map = HashMap::new();
    for image in images {
        image_map.insert(image.id.clone(), image);
    }

    let mut responses = Vec::with_capacity(image_ids.len());
    for image_id in image_ids {
        if let Some(image) = image_map.get(&image_id) {
            responses.push(ProjectRenderingImageResponse::from(image));
        }
    }

    Ok(Json(ProjectRenderingsResponse::new(responses)))
}

pub async fn get_room_items(
    State(state): State<AppState>,
    Path(room_key): Path<String>,
) -> Result<Json<RoomItemsResponse>, ApiError> {
    let (project_id_raw, room_id) = room_key
        .split_once('_')
        .ok_or_else(|| ApiError::not_found(format!("invalid room identifier: {room_key}")))?;

    if project_id_raw.is_empty() || room_id.is_empty() {
        return Err(ApiError::not_found(format!(
            "invalid room identifier: {room_key}"
        )));
    }

    let project_repository = state.project_repository()?;
    let mut project = project_repository.get_by_id(project_id_raw).await?;
    populate_floorplans(&state.http_client, &state.cdn_base_url, &mut project).await?;

    let Some((floorplan, room)) = project.floorplans.iter().find_map(|floorplan| {
        floorplan.rooms.as_ref().and_then(|rooms| {
            rooms
                .iter()
                .find(|room| room.archi_id == room_id)
                .map(|room| (floorplan, room))
        })
    }) else {
        return Err(ApiError::not_found(format!(
            "room {room_id} not found in project {project_id_raw}"
        )));
    };

    let room_item_ids: HashSet<String> = room
        .items
        .iter()
        .filter_map(|item| item.archi_id.clone())
        .collect();

    let mut items = floorplan
        .items
        .as_ref()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    item.archi_id
                        .as_ref()
                        .map(|id| room_item_ids.contains(id))
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if items.is_empty() && !room.items.is_empty() {
        items = room.items.clone();
    }

    let response = RoomItemsResponse::try_from_project_room(&project, floorplan, room, items)
        .map_err(ApiError::internal)?;

    Ok(Json(response))
}

pub async fn create_project_structure(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<AxumStatusCode, ApiError> {
    let project_repository = state.project_repository()?;
    let floor_structure_repository = state.floor_structure_repository()?;
    let room_structure_repository = state.room_structure_repository()?;

    let mut project = project_repository.get_by_id(&project_id).await?;
    populate_floorplans(&state.http_client, &state.cdn_base_url, &mut project).await?;

    let (floor_records, room_records) = build_structure_records(&project_id, &project.floorplans)?;

    floor_structure_repository.save_all(floor_records).await?;
    room_structure_repository.save_all(room_records).await?;

    Ok(AxumStatusCode::NO_CONTENT)
}

pub async fn create_recent_project_structures(
    State(state): State<AppState>,
) -> Result<AxumStatusCode, ApiError> {
    const RECENT_LIMIT: i64 = 300;

    let project_repository = state.project_repository()?;
    let floor_structure_repository = state.floor_structure_repository()?;
    let room_structure_repository = state.room_structure_repository()?;

    let ids = project_repository.find_recent_ids(RECENT_LIMIT).await?;

    let mut floor_records = Vec::new();
    let mut room_records = Vec::new();

    for project_id in ids {
        let mut project = project_repository.get_by_id(&project_id).await?;
        populate_floorplans(&state.http_client, &state.cdn_base_url, &mut project).await?;

        if project.floorplans.is_empty() {
            continue;
        }

        let (project_floor_records, project_room_records) =
            build_structure_records(&project_id, &project.floorplans)?;

        floor_records.extend(project_floor_records);
        room_records.extend(project_room_records);
    }

    floor_structure_repository.save_all(floor_records).await?;
    room_structure_repository.save_all(room_records).await?;

    Ok(AxumStatusCode::NO_CONTENT)
}

pub async fn get_similar_floors(
    State(state): State<AppState>,
    Path(floor_id): Path<String>,
    Query(query): Query<AreaRangeQuery>,
) -> Result<Json<Vec<FloorResponse>>, ApiError> {
    const SIMILAR_LIMIT: u64 = 10;

    let project_repository = state.project_repository()?;
    let floor_structure_repository = state.floor_structure_repository()?;
    let image_repository = state.image_repository()?;

    let floor = floor_structure_repository
        .find_by_id(&floor_id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("floor {floor_id} not found")))?;

    let area_from = query
        .area_from
        .map(|value| value as f64)
        .unwrap_or(floor.area * 0.85);
    let area_to = query
        .area_to
        .map(|value| value as f64)
        .unwrap_or(floor.area * 1.15);

    let similar_floors = floor_structure_repository
        .find_top_k_similar_floors(
            &floor.project_id,
            floor.area,
            floor.room_count,
            area_from,
            area_to,
            floor.bounding_box_aspect_ri,
            floor.rectangularity,
            SIMILAR_LIMIT,
        )
        .await?;

    if similar_floors.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let mut project_ids: Vec<String> = similar_floors
        .iter()
        .map(|record| record.project_id.clone())
        .collect();
    project_ids.sort();
    project_ids.dedup();

    let mut projects = project_repository.find_many_by_ids(&project_ids).await?;
    for project in &mut projects {
        ensure_default_cover_image(
            project_repository,
            project,
            state.s3_client.as_ref(),
            state.s3_bucket.as_deref(),
            &state.cdn_base_url,
        )
        .await?;
    }

    let mut project_map: HashMap<String, Project> = HashMap::new();
    for project in projects {
        if let Some(id) = project.id.clone() {
            project_map.insert(id, project);
        }
    }

    let mut all_image_ids = Vec::new();
    for project in project_map.values() {
        if let Some(ids) = &project.image_ids {
            all_image_ids.extend(ids.clone());
        }
    }
    all_image_ids.sort();
    all_image_ids.dedup();

    let images = image_repository.find_by_ids(&all_image_ids).await?;
    let mut image_map: HashMap<String, ProjectImage> = HashMap::new();
    for image in images {
        image_map.insert(image.id.clone(), image);
    }

    let mut responses = Vec::with_capacity(similar_floors.len());
    for record in similar_floors {
        if let Some(project) = project_map.get(&record.project_id) {
            let response = FloorResponse::try_from_project(
                project,
                &record.id,
                &record.title,
                &state.cdn_base_url,
                record.area,
                &image_map,
            )
            .map_err(ApiError::internal)?;
            responses.push(response);
        }
    }

    let mut responses_with_images = Vec::with_capacity(responses.len());
    let mut responses_without_images = Vec::new();
    for response in responses {
        if response.image_urls.is_empty() {
            responses_without_images.push(response);
        } else {
            responses_with_images.push(response);
        }
    }
    responses_with_images.extend(responses_without_images);

    Ok(Json(responses_with_images))
}

pub async fn get_similar_rooms(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Query(query): Query<AreaRangeQuery>,
) -> Result<Json<Vec<RoomResponse>>, ApiError> {
    let project_repository = state.project_repository()?;
    let room_structure_repository = state.room_structure_repository()?;
    let image_repository = state.image_repository()?;

    let room = room_structure_repository
        .find_by_id(&room_id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("room {room_id} not found")))?;

    let area_from = query
        .area_from
        .map(|value| (value as f64) * 1_000_000.0)
        .unwrap_or(room.area * 0.85);
    let area_to = query
        .area_to
        .map(|value| (value as f64) * 1_000_000.0)
        .unwrap_or(room.area * 1.15);

    let similar_rooms = room_structure_repository
        .find_similar_rooms(
            &room.project_id,
            room.area,
            area_from,
            area_to,
            room.rectangularity,
            room.bounding_box_aspect_ri,
            room.r#type,
        )
        .await?;

    if similar_rooms.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let mut project_ids: Vec<String> = similar_rooms
        .iter()
        .map(|record| record.project_id.clone())
        .collect();
    project_ids.sort();
    project_ids.dedup();

    let mut projects = project_repository.find_many_by_ids(&project_ids).await?;
    for project in &mut projects {
        ensure_default_cover_image(
            project_repository,
            project,
            state.s3_client.as_ref(),
            state.s3_bucket.as_deref(),
            &state.cdn_base_url,
        )
        .await?;
    }

    let mut project_map: HashMap<String, Project> = HashMap::new();
    for project in projects {
        if let Some(id) = project.id.clone() {
            project_map.insert(id, project);
        }
    }

    let mut all_image_ids = Vec::new();
    for project in project_map.values() {
        if let Some(ids) = &project.image_ids {
            all_image_ids.extend(ids.clone());
        }
    }
    all_image_ids.sort();
    all_image_ids.dedup();

    let images = image_repository.find_by_ids(&all_image_ids).await?;
    let mut image_map: HashMap<String, ProjectImage> = HashMap::new();
    for image in images {
        image_map.insert(image.id.clone(), image);
    }

    let mut responses = Vec::with_capacity(similar_rooms.len());
    for record in similar_rooms {
        if let Some(project) = project_map.get(&record.project_id) {
            let response = RoomResponse::try_from_project(
                project,
                &record.id,
                &state.cdn_base_url,
                record.area,
                &image_map,
            )
            .map_err(ApiError::internal)?;
            responses.push(response);
        }
    }

    let mut responses_with_images = Vec::with_capacity(responses.len());
    let mut responses_without_images = Vec::new();
    for response in responses {
        if response.image_urls.is_empty() {
            responses_without_images.push(response);
        } else {
            responses_with_images.push(response);
        }
    }
    responses_with_images.extend(responses_without_images);

    Ok(Json(responses_with_images))
}

async fn populate_floorplans(
    http_client: &HttpClient,
    cdn_base_url: &str,
    project: &mut Project,
) -> Result<(), ApiError> {
    let Some(path) = project.floorplan_path.as_ref() else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }

    let Some(key) = project.floorplan_key() else {
        return Ok(());
    };

    let url = format!(
        "{}/{}",
        cdn_base_url.trim_end_matches('/'),
        key.trim_start_matches('/')
    );
    let response = http_client
        .get(&url)
        .send()
        .await
        .map_err(ApiError::internal)?;

    if response.status() == HttpStatusCode::NOT_FOUND {
        return Ok(());
    }
    if !response.status().is_success() {
        return Err(ApiError::internal(anyhow::anyhow!(
            "failed to fetch floorplans from {url}: status {}",
            response.status()
        )));
    }

    let payload = response.text().await.map_err(ApiError::internal)?;
    let floorplans: Vec<Floorplan> = serde_json::from_str(&payload).map_err(ApiError::internal)?;
    project.floorplans = floorplans;
    Ok(())
}

async fn ensure_default_cover_image(
    repository: &ProjectRepository,
    project: &mut Project,
    s3_client: Option<&S3Client>,
    s3_bucket: Option<&str>,
    cdn_base_url: &str,
) -> Result<(), ApiError> {
    if project.default_cover_image.is_some() {
        return Ok(());
    }

    let cover_image = project
        .cover_image
        .clone()
        .ok_or_else(|| ApiError::internal(anyhow::anyhow!("project missing cover_image")))?;

    if let (Some(client), Some(bucket), Some(project_id)) =
        (s3_client, s3_bucket, project.id.clone())
    {
        let prefix = format!("projects/{project_id}/images");
        let response = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(&prefix)
            .send()
            .await
            .map_err(ApiError::internal)?;

        let contents = response.contents();
        if !contents.is_empty() {
            let mut latest_key = None;
            let mut latest_time = None;

            for object in contents {
                let key = object.key().map(ToOwned::to_owned);
                let modified = object.last_modified().cloned();
                match (key, modified) {
                    (Some(key), Some(modified)) => {
                        if latest_time.map(|t| t < modified).unwrap_or(true) {
                            latest_time = Some(modified);
                            latest_key = Some(key);
                        }
                    }
                    _ => continue,
                }
            }

            if let Some(key) = latest_key {
                let url = format!(
                    "{}/{}",
                    cdn_base_url.trim_end_matches('/'),
                    key.trim_start_matches('/')
                );
                project.default_cover_image = Some(url.clone());
                repository
                    .persist_default_cover_image(&project_id, &url)
                    .await?;
            }
        }
    }

    if project.default_cover_image.is_none() {
        project.default_cover_image = Some(cover_image);
        if let (Some(project_id), Some(default_image)) = (
            project.id.as_deref(),
            project.default_cover_image.as_deref(),
        ) {
            repository
                .persist_default_cover_image(project_id, default_image)
                .await?;
        }
    }

    Ok(())
}

fn build_structure_records(
    project_id: &str,
    floorplans: &[Floorplan],
) -> Result<(Vec<FloorStructureRecord>, Vec<RoomStructureRecord>), ApiError> {
    let floor_records = build_floor_structure_records(project_id, floorplans)?;
    let room_records = build_room_structure_records(project_id, floorplans)?;

    Ok((floor_records, room_records))
}

fn build_floor_structure_records(
    project_id: &str,
    floorplans: &[Floorplan],
) -> Result<Vec<FloorStructureRecord>, ApiError> {
    let mut records = Vec::with_capacity(floorplans.len());

    for floorplan in floorplans {
        let bounding_box = BoundingBox::from_floorplan(floorplan).map_err(ApiError::internal)?;
        let area = floorplan
            .area
            .ok_or_else(|| anyhow::anyhow!("floorplan {} missing area", floorplan.id))
            .map_err(ApiError::internal)?;
        let title = floorplan
            .title
            .clone()
            .ok_or_else(|| anyhow::anyhow!("floorplan {} missing title", floorplan.id))
            .map_err(ApiError::internal)?;
        let room_count = floorplan
            .rooms
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("floorplan {} missing rooms", floorplan.id))
            .map_err(ApiError::internal)?
            .len() as i32;
        let archi_id = floorplan.archi_id.clone();
        let rectangularity = if bounding_box.area > 0.0 {
            area * 1_000_000.0 / bounding_box.area
        } else {
            0.0
        };

        records.push(FloorStructureRecord {
            id: format!("{project_id}_{archi_id}"),
            title,
            project_id: project_id.to_string(),
            area,
            room_count,
            bounding_box_width: bounding_box.width,
            bounding_box_height: bounding_box.height,
            bounding_box_area: bounding_box.area,
            bounding_box_aspect: bounding_box.aspect,
            bounding_box_aspect_ri: bounding_box.aspect_ri,
            rectangularity,
        });
    }

    Ok(records)
}

fn build_room_structure_records(
    project_id: &str,
    floorplans: &[Floorplan],
) -> Result<Vec<RoomStructureRecord>, ApiError> {
    let mut records = Vec::new();

    for floorplan in floorplans {
        let rooms = floorplan
            .rooms
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("floorplan {} missing rooms", floorplan.id))
            .map_err(ApiError::internal)?;

        for room in rooms {
            let bounding_box =
                BoundingBox::from_room(floorplan, room).map_err(ApiError::internal)?;
            let rectangularity = if bounding_box.area > 0.0 {
                room.area / bounding_box.area
            } else {
                0.0
            };

            let archi_id = room.archi_id.clone();

            records.push(RoomStructureRecord {
                id: format!("{project_id}_{archi_id}"),
                project_id: project_id.to_string(),
                r#type: room.r#type,
                area: room.area,
                bounding_box_width: bounding_box.width,
                bounding_box_height: bounding_box.height,
                bounding_box_area: bounding_box.area,
                bounding_box_aspect: bounding_box.aspect,
                bounding_box_aspect_ri: bounding_box.aspect_ri,
                rectangularity,
            });
        }
    }

    Ok(records)
}
