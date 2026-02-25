use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub uuid: Option<String>,
    pub name: String,
    pub rank: Option<String>,
    pub monthlyrank: Option<String>,
    pub staffrank: Option<String>,
    pub rankcolor: Option<String>,
    pub bedwars_level: Option<u32>,
    pub final_kills: Option<u32>,
    pub final_deaths: Option<u32>,
    pub fkdr: Option<f32>,
    pub wins: Option<u32>,
    pub losses: Option<u32>,
    pub wlr: Option<f32>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct MojangResponse {
    id: String,
    name: String
}

#[derive(Deserialize, Serialize)]
struct HypixelResponse {
    player: Option<HypixelPlayer>
}

#[derive(Deserialize, Serialize)]
struct HypixelPlayer {
    #[serde(rename = "newPackageRank")]
    rank: Option<String>,
    #[serde(rename = "monthlyPackageRank")]
    monthly_rank: Option<String>,
    #[serde(rename = "rank")]
    staff_rank: Option<String>,
    #[serde(rename = "rankPlusColor")]
    plus_color: Option<String>,
    displayname: Option<String>,
    achievements: Option<HypixelAchievements>,
    stats: Option<HypixelStats>
}

#[derive(Deserialize, Serialize)]
struct HypixelAchievements {
    bedwars_level: Option<u32>
}

#[derive(Deserialize, Serialize)]
struct HypixelStats {
    #[serde(rename = "Bedwars")]
    bedwars: Option<HypixelStatsBedwars>
}

#[derive(Deserialize, Serialize)]
struct HypixelStatsBedwars {
    final_kills_bedwars: Option<u32>,
    final_deaths_bedwars: Option<u32>,
    wins_bedwars: Option<u32>,
    losses_bedwars: Option<u32>
}

impl Player {
    pub async fn get_uuid(&self) -> Option<String> {
        let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", self.name);

        let response = match reqwest::get(&url).await {
            Ok(r) => r,
            Err(e) => { if e.is_connect() || e.is_timeout() {
                    // update Mojang status to offline
                    println!("[networking::get_uuid] Mojang offline");
                    return None;
                } else {
                    // update Mojang status to error
                    println!("[networking::get_uuid] Mojang error 1");
                    return None
                }
            }
        };

        match response.status() {
            reqwest::StatusCode::OK => {
                let profile = response.json::<MojangResponse>().await;
                match profile {
                    Ok(p) => {
                        // TODO: Emit Mojang ONLINE
                        println!("[networking::get_uuid] Mojang online (200)");
                        return Some(p.id);
                    },
                    Err(_) => {
                        // TODO: Emit "INVALID_RESPONSE" event
                        println!("[networking::get_uuid] Mojang parsing error");
                        return None;
                    }
                }
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                // TODO: Emit "RATELIMIT" event to frontend
                println!("[networking::get_uuid] Mojang ratelimit");
                return None;
            },
            _ => {
                // TODO: Emit generic error event with status code
                println!("[networking::get_uuid] Mojang error 2");
                return None
            }
        }
    }

    pub async fn get_hypixel_player(&mut self, apikey: String) {
        if self.uuid.is_none() {
            return;
        }

        let url = format!("https://api.hypixel.net/player?key={}&uuid={}", apikey, self.uuid.as_ref().unwrap());

        let response = match reqwest::get(&url).await {
            Ok(r) => r,
            Err(e) => { if e.is_connect() || e.is_timeout() {
                    // update Hypixel status to offline
                    println!("[networking::get_hypixel_player] Hypixel offline");
                    return;
                } else {
                    // update Hypixel status to error
                    println!("[networking::get_hypixel_player] Hypixel error 1");
                    return;
                }
            }
        };

        match response.status() {
            reqwest::StatusCode::OK => {
                let profile = response.json::<HypixelResponse>().await;
                match profile {
                    Ok(p) => {
                        println!("[networking::get_hypixel_player] Hypixel online (200)");
                        if p.player.is_none() {
                            return;
                        }
                        let player = p.player.unwrap();
                        if player.rank.is_some() {
                            self.rank = player.rank;
                        }
                        if player.monthly_rank.is_some() {
                            self.monthlyrank = player.monthly_rank;
                        }
                        if player.staff_rank.is_some() {
                            self.staffrank = player.staff_rank;
                        }
                        if player.plus_color.is_some() {
                            self.rankcolor = player.plus_color;
                        }
                        
                        if player.displayname.is_some() {
                            self.name = player.displayname.unwrap();
                        }

                        if player.achievements.is_some() {
                            let achievements = player.achievements.unwrap();
                            
                            if achievements.bedwars_level.is_some() {
                                self.bedwars_level = achievements.bedwars_level;
                            }
                        }

                        if player.stats.is_some() {
                            let stats = player.stats.unwrap();

                            if stats.bedwars.is_some() {
                                let bedwars = stats.bedwars.unwrap();

                                if bedwars.final_kills_bedwars.is_some() {
                                    self.final_kills = bedwars.final_kills_bedwars;
                                }

                                if bedwars.wins_bedwars.is_some() {
                                    self.wins = bedwars.wins_bedwars;
                                }

                                if bedwars.final_deaths_bedwars.is_some() && self.final_kills.is_some() {
                                    let mut final_deaths = bedwars.final_deaths_bedwars.unwrap();

                                    if final_deaths < 1 {
                                        final_deaths = 1; // prevent division by 0
                                    }

                                    self.fkdr = Some(((self.final_kills.unwrap() as f32 / final_deaths as f32)*100.0).round() / 100.0);
                                }

                                if bedwars.losses_bedwars.is_some() && self.wins.is_some() {
                                    let mut losses = bedwars.losses_bedwars.unwrap();

                                    if losses < 1 {
                                        losses = 1;
                                    }

                                    self.wlr = Some(((self.wins.unwrap() as f32 / losses as f32)*100.0).round() / 100.0);
                                }
                            }
                        }

                    },
                    Err(_) => {
                        // TODO: Emit "INVALID_RESPONSE" event
                        println!("[networking::get_hypixel_player] Hypixel parsing error");
                        return;
                    }
                }
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                // TODO: Emit "RATELIMIT" event to frontend
                println!("[networking::get_hypixel_player] Hypixel ratelimit");
                return;
            },
            _ => {
                // TODO: Emit generic error event with status code
                println!("[networking::get_hypixel_player] Hypixel error 2");
                return;
            }
        }
    }
}