import { invoke } from "@tauri-apps/api/core";
import "./logo.css";

function Logo() {
    const close_all_menus = () => {
        //TODO close all menus
        invoke("close_general_settings");
    }

    const toggle_sidebar = () => {
        close_all_menus()
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