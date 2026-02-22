import "./topbar.css";
import Logo from "./logo";
import Buttons from "./Buttons";
import PlayerSearch from "./PlayerSearch";

function Topbar() {
    return (
        <div data-tauri-drag-region className="topbar">
            <Logo />
            <PlayerSearch />
            <Buttons />
        </div>
    );
}

export default Topbar;