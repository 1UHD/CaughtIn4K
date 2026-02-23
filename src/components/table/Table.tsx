import { useEffect, useState } from "react";
import "./table.css"
import { formatRank, formatStars } from "../../functional/statsFormatter";

interface PlayerProps {
    uuid: string;
    name: string;
    formatted_name: string | undefined;
    hypixel_level: number | undefined;
    bedwars_level: string | undefined;
    winstreak: number | undefined;
    final_kills: number | undefined;
    final_deaths: number | undefined;
    fkdr: number | undefined;
    kills: number | undefined;
    deaths: number | undefined;
    kdr: number | undefined;
    beds_broken: number | undefined;
    beds_lost: number | undefined;
    bblr: number | undefined;
    wins: number | undefined;
    losses: number | undefined;
    wlr: number | undefined;
}

function Player({ uuid, formatted_name, bedwars_level, final_kills, fkdr, wins, wlr }: PlayerProps) {
    let attributes = [bedwars_level, formatted_name, final_kills, fkdr, wins, wlr];

    const player_skull = `https://mc-heads.net/avatar/${uuid}`;

    return (
        <tr key={uuid}>
            <td><img src={player_skull} /></td>
            {attributes.map((item) => (
                <td>{item}</td>
            ))}
        </tr>
    )
}

function Table() {
    const attributes = ["LEVEL", "NAME", "FINALS", "FKDR", "WINS", "WLR"];
    const [players, setPlayers] = useState<PlayerProps[]>([]);
    
    useEffect(() => {
        setPlayers([
            {
                uuid: "860d353d-1f1e-4356-a059-fec025a2b590",
                name: "iUHD",
                formatted_name: formatRank("MVP_PLUS", "NONE", "NONE","DARK_GRAY", "iUHD"),
                hypixel_level: 0,
                bedwars_level: formatStars(3475),
                winstreak: 1,
                final_kills: 5481,
                final_deaths: 1038,
                fkdr: 1.15,
                kills: 0,
                deaths: 0,
                kdr: 0,
                wins: 2019,
                losses: 5021,
                wlr: 0.38,
                beds_broken: 1939,
                beds_lost: 3401,
                bblr: 0.69,
            }
        ])
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
                    players ? <Player {...player} /> : null
                ))}
            </table>
        </div>
    );
}

export default Table;