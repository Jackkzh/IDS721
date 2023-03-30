use spotify_web_api::{Client, Spotify};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 提示用户输入歌手名称
    println!("请输入歌手名称：");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let artist_name = input.trim();

    // 设置 API 密钥
    let api_key = "you-api-key";

    // 创建 Spotify API 客户端
    let spotify = Client::new().auth_token(api_key).build();

    // 搜索歌手
    let artist = spotify.search().artist(artist_name).limit(1).get().await?.artists.items.first().ok_or("Artist not found")?;

    // 获取歌手演出的列表
    let artist_shows = spotify.artist().shows(&artist.id).limit(10).get().await?;

    // 打印歌手演出的信息
    for show in artist_shows.items {
        println!("{} - {}", show.name, show.date);
    }

    Ok(())
}


