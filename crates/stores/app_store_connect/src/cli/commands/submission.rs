use crate::cli::GlobalArgs;
use crate::cli::commands::app::resolve_app;
use crate::types as asc;
use crate::{AppStoreConnectContext, print_json, print_table};
use anyhow::{Context as _, Result, anyhow};
use clap::{Args, Subcommand};
use serde::Serialize;
use std::time::{Duration, Instant};

const PLATFORMS: [&str; 4] = ["IOS", "MAC_OS", "TV_OS", "VISION_OS"];
const STATES: [&str; 7] = [
    "READY_FOR_REVIEW",
    "WAITING_FOR_REVIEW",
    "IN_REVIEW",
    "UNRESOLVED_ISSUES",
    "CANCELING",
    "COMPLETING",
    "COMPLETE",
];

#[derive(Args, Debug)]
pub struct SubmissionArgs {
    #[command(subcommand)]
    pub command: SubmissionCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubmissionCommand {
    #[command(about = "List review submissions for an app")]
    List(SubmissionListArgs),
    #[command(about = "View a review submission")]
    View(SubmissionViewArgs),
    #[command(about = "Create a draft review submission")]
    Create(SubmissionCreateArgs),
    #[command(about = "List the items in a review submission")]
    Items(SubmissionItemsArgs),
    #[command(name = "add-item", about = "Add an item to a review submission")]
    AddItem(SubmissionAddItemArgs),
    #[command(
        name = "remove-item",
        about = "Remove an item from a review submission"
    )]
    RemoveItem(SubmissionRemoveItemArgs),
    #[command(about = "Submit a review submission to App Review")]
    Submit(SubmissionSubmitArgs),
    #[command(about = "Cancel a submitted review submission")]
    Cancel(SubmissionCancelArgs),
}

#[derive(Args, Debug)]
pub struct SubmissionListArgs {
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "platform", value_parser = PLATFORMS)]
    pub platform: Option<String>,
    #[arg(long = "state", value_parser = STATES)]
    pub state: Option<String>,
}

#[derive(Args, Debug)]
pub struct SubmissionViewArgs {
    #[arg(value_name = "SUBMISSION_ID")]
    pub submission_id: String,
}

#[derive(Args, Debug)]
pub struct SubmissionCreateArgs {
    #[arg(long = "app")]
    pub app: String,
    #[arg(long = "platform", value_parser = PLATFORMS)]
    pub platform: Option<String>,
}

#[derive(Args, Debug)]
pub struct SubmissionItemsArgs {
    #[arg(value_name = "SUBMISSION_ID")]
    pub submission_id: String,
}

#[derive(Args, Debug)]
pub struct SubmissionAddItemArgs {
    #[arg(value_name = "SUBMISSION_ID")]
    pub submission_id: String,
    /// App Store Connect resource type, for example `appStoreVersions`.
    #[arg(long = "item-type")]
    pub item_type: String,
    #[arg(long = "item-id")]
    pub item_id: String,
}

#[derive(Args, Debug)]
pub struct SubmissionRemoveItemArgs {
    #[arg(value_name = "ITEM_ID")]
    pub item_id: String,
}

#[derive(Args, Debug)]
pub struct SubmissionSubmitArgs {
    #[arg(value_name = "SUBMISSION_ID")]
    pub submission_id: String,
    #[arg(long = "wait", default_value_t = false)]
    pub wait: bool,
    #[arg(long = "timeout", default_value = "30m")]
    pub timeout: String,
}

#[derive(Args, Debug)]
pub struct SubmissionCancelArgs {
    #[arg(value_name = "SUBMISSION_ID")]
    pub submission_id: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct SubmissionRow {
    pub id: String,
    pub platform: String,
    pub state: String,
    #[serde(rename = "submittedDate")]
    pub submitted_date: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct SubmissionItemRow {
    pub id: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub state: String,
}

pub async fn execute(args: &SubmissionArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = AppStoreConnectContext::from_env()?;
    match &args.command {
        SubmissionCommand::List(args) => list(&ctx, args, global).await,
        SubmissionCommand::View(args) => view(&ctx, args, global).await,
        SubmissionCommand::Create(args) => create(&ctx, args, global).await,
        SubmissionCommand::Items(args) => items(&ctx, args, global).await,
        SubmissionCommand::AddItem(args) => add_item_command(&ctx, args, global).await,
        SubmissionCommand::RemoveItem(args) => remove_item(&ctx, args).await,
        SubmissionCommand::Submit(args) => submit(&ctx, args, global).await,
        SubmissionCommand::Cancel(args) => cancel(&ctx, args, global).await,
    }
}

async fn list(
    ctx: &AppStoreConnectContext,
    args: &SubmissionListArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let filter_platform = args
        .platform
        .as_deref()
        .map(str::parse::<asc::AppsReviewSubmissionsGetToManyRelatedFilterPlatformItem>)
        .transpose()
        .map_err(|_| anyhow!("invalid submission platform"))?
        .map(|value| vec![value]);
    let filter_state = args
        .state
        .as_deref()
        .map(str::parse::<asc::AppsReviewSubmissionsGetToManyRelatedFilterStateItem>)
        .transpose()
        .map_err(|_| anyhow!("invalid submission state"))?
        .map(|value| vec![value]);
    let response = ctx
        .client
        .apps_review_submissions_get_to_many_related(
            &app.id,
            None,
            None,
            None,
            None,
            None,
            filter_platform.as_ref(),
            filter_state.as_ref(),
            None,
            global.limit,
            None,
        )
        .await
        .map_err(|e| anyhow!("failed to list review submissions: {e}"))?;
    let submissions = response
        .into_inner()
        .data
        .into_iter()
        .map(submission_row)
        .collect::<Result<Vec<_>>>()?;
    print_submissions(submissions, global)
}

async fn view(
    ctx: &AppStoreConnectContext,
    args: &SubmissionViewArgs,
    global: &GlobalArgs,
) -> Result<()> {
    print_submissions(
        vec![get_submission(ctx, &args.submission_id).await?],
        global,
    )
}

async fn create(
    ctx: &AppStoreConnectContext,
    args: &SubmissionCreateArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let app = resolve_app(ctx, &args.app).await?;
    let submission = create_submission(ctx, &app.id, args.platform.as_deref()).await?;
    print_submissions(vec![submission], global)
}

async fn items(
    ctx: &AppStoreConnectContext,
    args: &SubmissionItemsArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let response = ctx
        .client
        .review_submissions_items_get_to_many_related(
            &args.submission_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            global.limit,
        )
        .await
        .map_err(|e| anyhow!("failed to list review submission items: {e}"))?;
    let items = response
        .into_inner()
        .data
        .into_iter()
        .map(submission_item_row)
        .collect::<Result<Vec<_>>>()?;
    print_submission_items(items, global)
}

async fn add_item_command(
    ctx: &AppStoreConnectContext,
    args: &SubmissionAddItemArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let item =
        add_submission_item(ctx, &args.submission_id, &args.item_type, &args.item_id).await?;
    print_submission_items(vec![item], global)
}

async fn remove_item(ctx: &AppStoreConnectContext, args: &SubmissionRemoveItemArgs) -> Result<()> {
    ctx.client
        .review_submission_items_delete_instance(&args.item_id)
        .await
        .map_err(|e| anyhow!("failed to remove review submission item: {e}"))?;
    println!("Removed review submission item {}", args.item_id);
    Ok(())
}

async fn submit(
    ctx: &AppStoreConnectContext,
    args: &SubmissionSubmitArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let mut submission = submit_submission(ctx, &args.submission_id).await?;
    if args.wait {
        submission =
            wait_for_submission(ctx, &args.submission_id, parse_duration(&args.timeout)?).await?;
    }
    print_submissions(vec![submission], global)
}

async fn cancel(
    ctx: &AppStoreConnectContext,
    args: &SubmissionCancelArgs,
    global: &GlobalArgs,
) -> Result<()> {
    let submission = update_submission(ctx, &args.submission_id, false).await?;
    print_submissions(vec![submission], global)
}

pub async fn create_submission(
    ctx: &AppStoreConnectContext,
    app_id: &str,
    platform: Option<&str>,
) -> Result<SubmissionRow> {
    let platform = platform
        .map(str::parse::<asc::Platform>)
        .transpose()
        .map_err(|_| anyhow!("invalid submission platform"))?;
    let body = asc::ReviewSubmissionCreateRequest {
        data: asc::ReviewSubmissionCreateRequestData {
            attributes: platform.map(
                |platform| asc::ReviewSubmissionCreateRequestDataAttributes {
                    platform: Some(platform),
                },
            ),
            relationships: asc::ReviewSubmissionCreateRequestDataRelationships {
                app: asc::ReviewSubmissionCreateRequestDataRelationshipsApp {
                    data: asc::ReviewSubmissionCreateRequestDataRelationshipsAppData {
                        id: app_id.to_string(),
                        type_: asc::ReviewSubmissionCreateRequestDataRelationshipsAppDataType::Apps,
                    },
                },
            },
            type_: asc::ReviewSubmissionCreateRequestDataType::ReviewSubmissions,
        },
    };
    let response = ctx
        .client
        .review_submissions_create_instance(&body)
        .await
        .map_err(|e| anyhow!("failed to create review submission: {e}"))?;
    submission_row(response.into_inner().data)
}

pub async fn add_submission_item(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
    item_type: &str,
    item_id: &str,
) -> Result<SubmissionItemRow> {
    let body = build_item_request(submission_id, item_type, item_id)?;
    let response = ctx
        .client
        .review_submission_items_create_instance(&body)
        .await
        .map_err(|e| anyhow!("failed to add review submission item: {e}"))?;
    submission_item_row(response.into_inner().data)
}

pub async fn submit_submission(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
) -> Result<SubmissionRow> {
    update_submission(ctx, submission_id, true).await
}

async fn update_submission(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
    submit: bool,
) -> Result<SubmissionRow> {
    let attributes = if submit {
        asc::ReviewSubmissionUpdateRequestDataAttributes {
            submitted: Some(true),
            ..Default::default()
        }
    } else {
        asc::ReviewSubmissionUpdateRequestDataAttributes {
            canceled: Some(true),
            ..Default::default()
        }
    };
    let body = asc::ReviewSubmissionUpdateRequest {
        data: asc::ReviewSubmissionUpdateRequestData {
            attributes: Some(attributes),
            id: submission_id.to_string(),
            type_: asc::ReviewSubmissionUpdateRequestDataType::ReviewSubmissions,
        },
    };
    let response = ctx
        .client
        .review_submissions_update_instance(submission_id, &body)
        .await
        .map_err(|e| anyhow!("failed to update review submission: {e}"))?;
    submission_row(response.into_inner().data)
}

pub async fn get_submission(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
) -> Result<SubmissionRow> {
    let response = ctx
        .client
        .review_submissions_get_instance(submission_id, None, None, None, None, None, None, None)
        .await
        .map_err(|e| anyhow!("failed to fetch review submission: {e}"))?;
    submission_row(response.into_inner().data)
}

pub async fn wait_for_submission(
    ctx: &AppStoreConnectContext,
    submission_id: &str,
    timeout: Duration,
) -> Result<SubmissionRow> {
    let start = Instant::now();
    loop {
        let submission = get_submission(ctx, submission_id).await?;
        match submission.state.as_str() {
            "WAITING_FOR_REVIEW" | "IN_REVIEW" | "COMPLETING" | "COMPLETE" => {
                return Ok(submission);
            }
            "UNRESOLVED_ISSUES" => {
                return Err(anyhow!(
                    "submission {submission_id} has unresolved review issues"
                ));
            }
            "CANCELING" => {
                return Err(anyhow!("submission {submission_id} is being canceled"));
            }
            _ if start.elapsed() >= timeout => {
                return Err(anyhow!(
                    "submission {submission_id} timed out after {:?}",
                    timeout
                ));
            }
            _ => tokio::time::sleep(Duration::from_secs(30)).await,
        }
    }
}

fn build_item_request(
    submission_id: &str,
    item_type: &str,
    item_id: &str,
) -> Result<asc::ReviewSubmissionItemCreateRequest> {
    let mut relationships = asc::ReviewSubmissionItemCreateRequestDataRelationships {
        app_custom_product_page_version: None,
        app_event: None,
        app_store_version: None,
        app_store_version_experiment: None,
        app_store_version_experiment_v2: None,
        background_asset_version: None,
        game_center_achievement_version: None,
        game_center_activity_version: None,
        game_center_challenge_version: None,
        game_center_leaderboard_set_version: None,
        game_center_leaderboard_version: None,
        review_submission:
            asc::ReviewSubmissionItemCreateRequestDataRelationshipsReviewSubmission {
                data: asc::ReviewSubmissionItemCreateRequestDataRelationshipsReviewSubmissionData {
                    id: submission_id.to_string(),
                    type_: asc::ReviewSubmissionItemCreateRequestDataRelationshipsReviewSubmissionDataType::ReviewSubmissions,
                },
            },
    };

    macro_rules! set_relationship {
        ($field:ident, $wrapper:ident, $data:ident, $data_type:ident, $variant:ident) => {
            relationships.$field = Some(asc::$wrapper {
                data: Some(asc::$data {
                    id: item_id.to_string(),
                    type_: asc::$data_type::$variant,
                }),
            })
        };
    }

    match item_type {
        "appStoreVersions" => set_relationship!(
            app_store_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionDataType,
            AppStoreVersions
        ),
        "appCustomProductPageVersions" => set_relationship!(
            app_custom_product_page_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppCustomProductPageVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppCustomProductPageVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppCustomProductPageVersionDataType,
            AppCustomProductPageVersions
        ),
        "appStoreVersionExperiments" => set_relationship!(
            app_store_version_experiment,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperiment,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperimentData,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperimentDataType,
            AppStoreVersionExperiments
        ),
        "appStoreVersionExperimentsV2" => set_relationship!(
            app_store_version_experiment_v2,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperimentV2,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperimentV2Data,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppStoreVersionExperimentV2DataType,
            AppStoreVersionExperiments
        ),
        "appEvents" => set_relationship!(
            app_event,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppEvent,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppEventData,
            ReviewSubmissionItemCreateRequestDataRelationshipsAppEventDataType,
            AppEvents
        ),
        "backgroundAssetVersions" => set_relationship!(
            background_asset_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsBackgroundAssetVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsBackgroundAssetVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsBackgroundAssetVersionDataType,
            BackgroundAssetVersions
        ),
        "gameCenterAchievementVersions" => set_relationship!(
            game_center_achievement_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterAchievementVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterAchievementVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterAchievementVersionDataType,
            GameCenterAchievementVersions
        ),
        "gameCenterActivityVersions" => set_relationship!(
            game_center_activity_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterActivityVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterActivityVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterActivityVersionDataType,
            GameCenterActivityVersions
        ),
        "gameCenterChallengeVersions" => set_relationship!(
            game_center_challenge_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterChallengeVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterChallengeVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterChallengeVersionDataType,
            GameCenterChallengeVersions
        ),
        "gameCenterLeaderboardSetVersions" => set_relationship!(
            game_center_leaderboard_set_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardSetVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardSetVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardSetVersionDataType,
            GameCenterLeaderboardSetVersions
        ),
        "gameCenterLeaderboardVersions" => set_relationship!(
            game_center_leaderboard_version,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardVersion,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardVersionData,
            ReviewSubmissionItemCreateRequestDataRelationshipsGameCenterLeaderboardVersionDataType,
            GameCenterLeaderboardVersions
        ),
        _ => {
            return Err(anyhow!(
                "unsupported submission item type `{item_type}`; use an App Store Connect resource type such as `appStoreVersions`"
            ));
        }
    }

    Ok(asc::ReviewSubmissionItemCreateRequest {
        data: asc::ReviewSubmissionItemCreateRequestData {
            relationships,
            type_: asc::ReviewSubmissionItemCreateRequestDataType::ReviewSubmissionItems,
        },
    })
}

fn print_submissions(submissions: Vec<SubmissionRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&submissions, global.json.as_deref());
    }
    let rows = submissions
        .into_iter()
        .map(|submission| {
            vec![
                submission.id,
                submission.platform,
                submission.state,
                submission.submitted_date,
            ]
        })
        .collect::<Vec<_>>();
    print_table(&["ID", "PLATFORM", "STATE", "SUBMITTED"], &rows);
    Ok(())
}

fn print_submission_items(items: Vec<SubmissionItemRow>, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&items, global.json.as_deref());
    }
    let rows = items
        .into_iter()
        .map(|item| vec![item.id, item.item_type, item.item_id, item.state])
        .collect::<Vec<_>>();
    print_table(&["ID", "TYPE", "RESOURCE_ID", "STATE"], &rows);
    Ok(())
}

fn submission_row(value: asc::ReviewSubmission) -> Result<SubmissionRow> {
    let attrs = value
        .attributes
        .ok_or_else(|| anyhow!("missing submission attributes"))?;
    Ok(SubmissionRow {
        id: value.id,
        platform: attrs
            .platform
            .map(|value| value.to_string())
            .unwrap_or_default(),
        state: attrs
            .state
            .map(|value| value.to_string())
            .unwrap_or_default(),
        submitted_date: attrs
            .submitted_date
            .map(|value| value.to_rfc3339())
            .unwrap_or_default(),
    })
}

fn submission_item_row(value: asc::ReviewSubmissionItem) -> Result<SubmissionItemRow> {
    let state = value
        .attributes
        .and_then(|attrs| attrs.state)
        .map(|value| value.to_string())
        .unwrap_or_default();
    let relationships = value
        .relationships
        .ok_or_else(|| anyhow!("missing submission item relationships"))?;

    macro_rules! related_resource {
        ($field:expr, $item_type:literal) => {
            if let Some(item_id) = $field
                .and_then(|relationship| relationship.data)
                .map(|data| data.id)
            {
                return Ok(SubmissionItemRow {
                    id: value.id,
                    item_type: $item_type.to_string(),
                    item_id,
                    state,
                });
            }
        };
    }

    related_resource!(relationships.app_store_version, "appStoreVersions");
    related_resource!(
        relationships.app_custom_product_page_version,
        "appCustomProductPageVersions"
    );
    related_resource!(
        relationships.app_store_version_experiment,
        "appStoreVersionExperiments"
    );
    related_resource!(
        relationships.app_store_version_experiment_v2,
        "appStoreVersionExperimentsV2"
    );
    related_resource!(relationships.app_event, "appEvents");
    related_resource!(
        relationships.background_asset_version,
        "backgroundAssetVersions"
    );
    related_resource!(
        relationships.game_center_achievement_version,
        "gameCenterAchievementVersions"
    );
    related_resource!(
        relationships.game_center_activity_version,
        "gameCenterActivityVersions"
    );
    related_resource!(
        relationships.game_center_challenge_version,
        "gameCenterChallengeVersions"
    );
    related_resource!(
        relationships.game_center_leaderboard_set_version,
        "gameCenterLeaderboardSetVersions"
    );
    related_resource!(
        relationships.game_center_leaderboard_version,
        "gameCenterLeaderboardVersions"
    );
    Err(anyhow!("missing related resource for submission item"))
}

fn parse_duration(value: &str) -> Result<Duration> {
    let value = value.trim();
    let (number, multiplier) = if let Some(number) = value.strip_suffix('m') {
        (number, 60)
    } else if let Some(number) = value.strip_suffix('s') {
        (number, 1)
    } else {
        (value, 1)
    };
    let number: u64 = number
        .parse()
        .with_context(|| format!("invalid duration: {value}"))?;
    Ok(Duration::from_secs(number * multiplier))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        args: SubmissionArgs,
    }

    #[test]
    fn converts_generated_submission_type() {
        let value = asc::ReviewSubmission {
            attributes: Some(asc::ReviewSubmissionAttributes {
                platform: Some(asc::Platform::Ios),
                state: Some(asc::ReviewSubmissionAttributesState::WaitingForReview),
                submitted_date: Some("2026-07-17T10:00:00Z".parse().unwrap()),
            }),
            id: "submission-1".to_string(),
            links: None,
            relationships: None,
            type_: asc::ReviewSubmissionType::ReviewSubmissions,
        };
        assert_eq!(
            submission_row(value).unwrap(),
            SubmissionRow {
                id: "submission-1".to_string(),
                platform: "IOS".to_string(),
                state: "WAITING_FOR_REVIEW".to_string(),
                submitted_date: "2026-07-17T10:00:00+00:00".to_string(),
            }
        );
    }

    #[test]
    fn builds_typed_app_store_version_item_request() {
        let request = build_item_request("submission-1", "appStoreVersions", "version-1").unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(
            value["data"]["relationships"]["appStoreVersion"]["data"]["id"],
            "version-1"
        );
        assert_eq!(
            value["data"]["relationships"]["reviewSubmission"]["data"]["id"],
            "submission-1"
        );
        assert!(build_item_request("submission-1", "unknownResources", "item-1").is_err());
    }

    #[test]
    fn parses_wait_timeout() {
        assert_eq!(parse_duration("30m").unwrap(), Duration::from_secs(1800));
        assert_eq!(parse_duration("45s").unwrap(), Duration::from_secs(45));
        assert!(parse_duration("soon").is_err());
    }

    #[test]
    fn parses_add_item_command() {
        let cli = TestCli::try_parse_from([
            "test",
            "add-item",
            "submission-1",
            "--item-type",
            "appStoreVersions",
            "--item-id",
            "version-1",
        ])
        .unwrap();
        let SubmissionCommand::AddItem(args) = cli.args.command else {
            panic!("expected add-item command");
        };
        assert_eq!(args.submission_id, "submission-1");
        assert_eq!(args.item_type, "appStoreVersions");
        assert_eq!(args.item_id, "version-1");
    }

    #[test]
    fn rejects_unknown_platform() {
        assert!(
            TestCli::try_parse_from([
                "test",
                "list",
                "--app",
                "com.example.app",
                "--platform",
                "ANDROID",
            ])
            .is_err()
        );
    }
}
