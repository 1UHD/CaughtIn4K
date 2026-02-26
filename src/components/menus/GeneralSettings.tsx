import "./generalsettings.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import SettingsCategory from "./components/SettingCategory";
import Toggle from "./components/Toggle";

function GeneralSettings() {
    const [generalSettingsState, setGeneralSettingsState] = useState<boolean>(false);

    const style_when_hidden = {
        right: "-100vw"
    };

    const style_when_visible = {
        right: "0px"
    };

    useEffect(() => {
        const unlisten_toggle_event = listen(
            "toggle-general-settings",
            () => {
                setGeneralSettingsState((state) => !state);
            }
        );

        const unlisten_close_event = listen(
            "close-general-settings",
            () => {
                setGeneralSettingsState(false);
            }
        );

        return () => {
            unlisten_toggle_event.then((unlisten) => unlisten());
            unlisten_close_event.then((unlisten) => unlisten());
        }
    }, []);

    return (
        <div className="general" style={generalSettingsState ? style_when_visible : style_when_hidden}>
            <SettingsCategory name="API">
                <p>Enable Caching</p>
                <Toggle default_state={true} />
            </SettingsCategory>
        </div>
    );
}

export default GeneralSettings;