import "./generalsettings.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import SettingsCategory from "./components/SettingCategory";
import Toggle from "./components/Toggle";
import InputBox from "./components/InputBox";
import { invoke } from "@tauri-apps/api/core";

function GeneralSettings() {
    const [generalSettingsState, setGeneralSettingsState] = useState<boolean>(false);
    const [apiKey, setApiKey] = useState<string>("");
    const [intervalMs, setIntervalMs] = useState<number>(1000);

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
                invoke("get_apikey");
                setGeneralSettingsState((state) => !state);
            }
        );

        const unlisten_close_event = listen(
            "close-general-settings",
            () => {
                setGeneralSettingsState(false);
            }
        );

        const unlisten_get_apikey = listen<string>(
            "get-apikey",
            (event) => {
                console.log(event.payload);
                setApiKey(event.payload);
            }
        );

        return () => {
            unlisten_toggle_event.then((unlisten) => unlisten());
            unlisten_close_event.then((unlisten) => unlisten());
            unlisten_get_apikey.then((unlisten) => unlisten());
        }
    }, []);

    const on_toggle_caching = (state: boolean) => {
        console.log(state);
    }

    const api_key_on_change = (event: any) => {
        if (event.key === "Enter") {
            setApiKey(event.target.value);
            invoke("write_apikey", { apikey: event.target.value });
            event.target.value = "";
        }
    }

    const interval_key_change = (event: any) => {
        if (event.key === "Enter") {
            setIntervalMs(Number(event.target.value));
            invoke("update_interval", { newms: Number(event.target.value) });
            event.target.value = "";
        }
    }

    const launch_fetcher = () => {
        invoke("initialize_fetcher");
    }

    const stop_fetcher = () => {
        invoke("stop_fetcher");
    }

    return (
        <div className="general" style={generalSettingsState ? style_when_visible : style_when_hidden}>
            <SettingsCategory name="API">
                <p>API Key</p>
                <InputBox on_key_down={api_key_on_change} placeholder={apiKey} privacy_box={true} />
                <p>Enable Caching</p>
                <Toggle default_state={false} event={on_toggle_caching}/>
            </SettingsCategory>
            <SettingsCategory name="FETCHING">
                <p onClick={launch_fetcher}>Init Fetcher</p>
                <div className="space" />
                <p onClick={stop_fetcher}>Stop Fetcher</p>
                <div className="space" />
                <p>Set Interval</p>
                <InputBox on_key_down={interval_key_change} placeholder={intervalMs.toString()} />
            </SettingsCategory>
        </div>
    );
}

export default GeneralSettings;