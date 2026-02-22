import "./playersearch.css";

function PlayerSearch() {
    let placeholder = "Search player";

    const searchPlayer = (event: any) => {
        if (event.key === "Enter") {
            //do shit
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