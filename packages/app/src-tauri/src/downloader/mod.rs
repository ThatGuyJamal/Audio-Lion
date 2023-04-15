use crate::{types::{AppError, VideoLinkType}, config::AppConfig};
use youtube_dl::{YoutubeDl};

/// Downloads the audio from a youtube video.
pub fn download_youtube_audio(
    video_url: &str,
    video_type_def: VideoLinkType,
     config: AppConfig
) -> Result<(), AppError> {
    let video_type = get_video_type(video_type_def);

    if video_type == "playlist" {
        let output = create_ytdl(video_url, config)
            .unwrap()
            .run()
            .map_err(|e| AppError {
                status: false,
                message: format!("Failed to download playlist: {}", e),
            })?
            .into_playlist()
            .unwrap();

        println!("Playlist: {:#?}", output);
    } else if video_type == "video" {
        let output = create_ytdl(video_url, config)
            .unwrap()
            .run()
            .map_err(|e| AppError {
                status: false,
                message: format!("Failed to download video: {}", e),
            })?
            .into_single_video()
            .unwrap();

        println!("Video: {:#?}", output);
    } else {
        return Err(AppError {
            status: false,
            message: format!("Invalid video type: {}", video_type),
        });
    }

    Ok(())
}

fn create_ytdl(video_url: &str, config: AppConfig) -> Result<YoutubeDl, AppError> {
    let mut ytdl = YoutubeDl::new(video_url);

    ytdl.extract_audio(true);
    ytdl.output_template("%(title)s.%(ext)s");
    ytdl.download(true);
    ytdl.format("mp3");
    ytdl.output_directory(config.download_folder.clone().unwrap_or_default());

    // Download the thumbnail for this audio as well.
    ytdl.extra_arg("--write-thumbnail");

    Ok(ytdl)
}

/// Gets the video type as a string.
fn get_video_type(video_type_def: VideoLinkType) -> &'static str {
    return match video_type_def {
        VideoLinkType::Playlist => "playlist",
        VideoLinkType::Video => "video",
    };
}
