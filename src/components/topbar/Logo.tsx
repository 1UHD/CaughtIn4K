import { invoke } from "@tauri-apps/api/core";
import "./logo.css";

function Logo() {
    const toggle_sidebar = () => {
        invoke("toggle_sidebar");
    }

    return (
        <div className="logo">
            <img src="logo/logo.png" id="logo" onClick={toggle_sidebar}/>
            <img data-tauri-drag-region src="logo/title.png" id="title"/>
        </div>
    );
}

export default Logo;