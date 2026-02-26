import { useEffect, useRef, useState } from "react";
import "./table.css"
import { formatRank, formatStars } from "../../functional/statsFormatter";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface PlayerProps {
    uuid: string | undefined;
    name: string;
    rank: string | undefined;
    monthlyrank: string | undefined;
    staffrank: string | undefined;
    rankcolor: string | undefined;
    bedwars_level: number | undefined;
    final_kills: number | undefined;
    final_deaths: number | undefined;
    fkdr: number | undefined;
    wins: number | undefined;
    losses: number | undefined;
    wlr: number | undefined;
}

function Player({ uuid, name, rank, monthlyrank, staffrank, rankcolor, bedwars_level, final_kills, fkdr, wins, wlr }: PlayerProps) {
    let display_name = formatRank(rank, monthlyrank, staffrank, rankcolor, name);
    let display_level = bedwars_level ? formatStars(bedwars_level) : "";

    let attributes = [display_level, display_name, final_kills, fkdr, wins, wlr];

    const player_skull = uuid ? `https://mc-heads.net/avatar/${uuid}` : `https://mc-heads.net/avatar/ff99328f-e0ca-45c2-8b86-969052b1d521`;

    const toggle_context_menu = (event: any) => {
        event.preventDefault();
        invoke("remove_player", { uuid });
    }

    return (
        <tr key={uuid} onContextMenu={toggle_context_menu}>
            <td>{player_skull ? <img src={player_skull} /> : null}</td>
            {attributes.map((item) => (
                <td>{item ? item : "-"}</td>
            ))}
        </tr>
    )
}

function Table() {
    const attributes = ["LEVEL", "NAME", "FINALS", "FKDR", "WINS", "WLR"];
    const [players, setPlayers] = useState<PlayerProps[]>([]);
    const playersRef = useRef<PlayerProps[]>(players);

    const player_already_exists = (uuid: string) => {
        return playersRef.current.some(p => p.uuid === uuid);
    }

    useEffect(() => {
        playersRef.current = players;
    }, [players]);
    
    useEffect(() => {
        const unlisten_add_player = listen<PlayerProps>(
            "add-player",
            async (event) => {
                const player_uuid = event.payload.uuid;

                if (player_uuid) {
                    console.log(player_already_exists(player_uuid));
                    if (player_already_exists(player_uuid)) {
                        return;
                    }
                }

                const player_stats = event.payload;
                setPlayers((prev_players) => [...prev_players, player_stats]);
            }
        );

        const unlisten_remove_player = listen<String>(
            "remove-player",
            (event) => {
                const player_uuid = event.payload;

                setPlayers((prev_players) => prev_players.filter(
                    (p) => p.uuid !== player_uuid
                ));
            }
        );

        const unlisten_clear_players = listen(
            "clear-players",
            () => {
                setPlayers([]);
            }
        );

        return () => {
            unlisten_add_player.then((unlisten) => unlisten());
            unlisten_remove_player.then((unlisten) => unlisten());
            unlisten_clear_players.then((unlisten) => unlisten());
        };
    }, []);
    

    return(
        <div className="table">
            <div className="space" />
            <table className="player-table">
                <tr>
                    <th id="player_skull"></th>
                    {attributes.map((header) => (
                        <th>{header}</th>
                    ))}
                </tr>
                {players.map((player) => (
                    players ? <Player key={player.uuid} {...player} /> : null
                ))}
            </table>
        </div>
    );
}

export default Table;