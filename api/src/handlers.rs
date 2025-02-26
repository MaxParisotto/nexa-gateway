// ...existing code...

pub async fn get_user_data(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserData>, AppError> {


    // Use real data service
    let user_data = state.user_service.get_user_data(&user_id).await?;
    Ok(Json(user_data))
}

// ...existing code...
