import { useEffect, useState } from "react";
import "./table.css"
import { formatRank, formatStars } from "../../functional/statsFormatter";

interface PlayerProps {
    uuid: string | undefined;
    name: string;
    rank: string | undefined;
    monthlyrank: string | undefined;
    staffrank: string | undefined;
    rankcolor: string | undefined;
    bedwars_level: number | undefined;
    final_kills: number | undefined;
    fkdr: number | undefined;
    wins: number | undefined;
    wlr: number | undefined;
}

function Player({ uuid, name, rank, monthlyrank, staffrank, rankcolor, bedwars_level, final_kills, fkdr, wins, wlr }: PlayerProps) {
    let display_name = formatRank(rank, monthlyrank, staffrank, rankcolor, name);
    let display_level = bedwars_level ? formatStars(bedwars_level) : "";

    let attributes = [display_level, display_name, final_kills, fkdr, wins, wlr];

    const player_skull = uuid ? `https://mc-heads.net/avatar/${uuid}` : `https://mc-heads.net/avatar/ff99328f-e0ca-45c2-8b86-969052b1d521`;

    return (
        <tr key={uuid}>
            <td>{player_skull ? <img src={player_skull} /> : null}</td>
            {attributes.map((item) => (
                <td>{item ? item : null}</td>
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
                rank: "MVP_PLUS",
                monthlyrank: "MVP_PLUS",
                staffrank: "NONE",
                rankcolor: "DARK_GRAY",
                bedwars_level: 1673,
                final_kills: 45481,
                fkdr: 10.15,
                wins: 10612,
                wlr: 3.38,
            },
            {
                uuid: undefined,
                name: "nicked_player",
                rank: undefined,
                monthlyrank: undefined,
                staffrank: undefined,
                rankcolor: undefined,
                bedwars_level: undefined,
                final_kills: undefined,
                fkdr: undefined,
                wins: undefined,
                wlr: undefined,
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