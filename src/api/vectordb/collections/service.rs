use std::sync::Arc;

use crate::{
    app_context::AppContext,
    models::collection::{Collection, CollectionIndexingStatus},
};

use super::{
    dtos::{
        CreateCollectionDto, CreateCollectionDtoResponse, GetCollectionsDto,
        GetCollectionsResponseDto,
    },
    error::CollectionsError,
    repo,
};

pub(crate) async fn create_collection(
    ctx: Arc<AppContext>,
    create_collection_dto: CreateCollectionDto,
) -> Result<CreateCollectionDtoResponse, CollectionsError> {
    let collection = repo::create_collection(ctx.clone(), create_collection_dto).await?;

    Ok(CreateCollectionDtoResponse {
        id: collection.meta.name.clone(),
        name: collection.meta.name.clone(),
        description: collection.meta.description.clone(),
    })
}

pub(crate) async fn get_collections(
    ctx: Arc<AppContext>,
    get_collections_dto: GetCollectionsDto,
) -> Result<Vec<GetCollectionsResponseDto>, CollectionsError> {
    let collections = repo::get_collections(ctx, get_collections_dto).await?;
    Ok(collections)
}

/// gets a collection by its id
///
/// currently collection_id = collection.name
pub(crate) async fn get_collection_by_id(
    ctx: Arc<AppContext>,
    collection_id: &str,
) -> Result<Arc<Collection>, CollectionsError> {
    let collection = repo::get_collection_by_name(ctx, collection_id).await?;
    Ok(collection)
}

pub(crate) async fn get_collection_indexing_status(
    ctx: Arc<AppContext>,
    collection_id: &str,
) -> Result<CollectionIndexingStatus, CollectionsError> {
    let status = repo::get_collection_indexing_status(ctx, collection_id).await?;
    Ok(status)
}

/// deletes a collection by its id
///
/// currently collection_id = collection.name
pub(crate) async fn delete_collection_by_id(
    ctx: Arc<AppContext>,
    collection_id: &str,
) -> Result<Arc<Collection>, CollectionsError> {
    let collection = repo::delete_collection_by_name(ctx, collection_id).await?;
    Ok(collection)
}

pub(crate) async fn load_collection(
    ctx: Arc<AppContext>,
    collection_id: &str,
) -> Result<Arc<Collection>, CollectionsError> {
    // First check if collection exists by loading its metadata
    let collection = repo::get_collection_by_name(ctx.clone(), collection_id).await?;

    // Then load it into the cache
    ctx.collection_cache_manager
        .load_collection(collection_id)
        .map_err(|e| CollectionsError::ServerError(format!("Failed to load collection: {}", e)))?;

    Ok(collection)
}

pub(crate) async fn unload_collection(
    ctx: Arc<AppContext>,
    collection_id: &str,
) -> Result<(), CollectionsError> {
    // Check if collection exists
    let _ = repo::get_collection_by_name(ctx.clone(), collection_id).await?;

    // Then unload it from the cache
    ctx.collection_cache_manager
        .unload_collection(collection_id)
        .map_err(|e| {
            CollectionsError::ServerError(format!("Failed to unload collection: {}", e))
        })?;

    Ok(())
}

pub(crate) async fn get_loaded_collections(
    ctx: Arc<AppContext>,
) -> Result<Vec<String>, CollectionsError> {
    // Just return the list of loaded collections directly
    Ok(ctx.collection_cache_manager.get_loaded_collections())
}

