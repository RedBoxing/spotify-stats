use std::time::Duration;

use serde::{Deserialize, Serialize};

// Number of StreamingHistory_music_* file you have
const HISTORY_FILE_NBR: i32 = 3;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct History {
    pub end_time: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u64,
}

#[derive(Debug)]
struct Track {
    pub artist: String,
    pub name: String,
    pub count: i32,
    pub total_played: Duration,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut history: Vec<History> = Vec::new();

    for i in 0..(HISTORY_FILE_NBR - 1) {
        let vec: Vec<History> = serde_json::from_str(&std::fs::read_to_string(format!(
            "data/StreamingHistory_music_{}.json",
            i
        ))?)?;

        for item in vec {
            history.push(item);
        }
    }

    println!("Got {} items in history", history.len());
    let mut tracks: Vec<Track> = Vec::new();

    for item in history {
        if let Some(track) = &mut tracks
            .iter_mut()
            .find(|track| track.artist == item.artist_name && track.name == item.track_name)
        {
            track.total_played += Duration::from_millis(item.ms_played);
            track.count += 1;
        } else {
            let new_track = Track {
                artist: item.artist_name,
                name: item.track_name,
                count: 1,
                total_played: Duration::from_millis(item.ms_played),
            };
            tracks.push(new_track);
        }
    }

    tracks.sort_by(|a, b| b.total_played.partial_cmp(&a.total_played).unwrap());

    let mut total_duration = Duration::from_millis(0);
    let mut total_count = 0;

    for track in tracks {
        println!("--------------------");
        println!("Track Name: {}", track.name);
        println!("Arist Name: {}", track.artist);

        let time_minute = track.total_played.as_secs_f64() / 60.0;
        if time_minute > 60.0 {
            println!("Total Play Time: {}h", time_minute / 60.0)
        } else {
            println!("Total Play Time: {}m", time_minute);
        }
        println!("Number of listens: {}", track.count);

        total_duration += track.total_played;
        total_count += track.count;
    }

    println!("-------------------");
    println!(
        "Total Play Time: {:?}h",
        total_duration.as_secs_f64() / 60.0 / 60.0
    );
    println!("Total Number of Listens: {}", total_count);

    Ok(())
}
