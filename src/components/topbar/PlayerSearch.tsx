import { invoke } from "@tauri-apps/api/core";
import "./playersearch.css";

function PlayerSearch() {
    let placeholder = "Search player";

    const searchPlayer = (event: any) => {
        if (event.key === "Enter") {
            invoke("req_player", { name: event.target.value });
            event.target.value = "";
        }
    }

    return (
        <div className="playersearch">
            <input
                type="text"
                id="search-input"
                placeholder={placeholder}
                onKeyDown={searchPlayer}
            />
        </div>
    );
}

export default PlayerSearch;