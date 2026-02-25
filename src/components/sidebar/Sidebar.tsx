import "./sidebar.css";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { gold, gray, green, red } from "../../functional/statsFormatter";

const status_styles: any = {
    "ONLINE": green,
    "RATELIMIT": gold,
    "ERROR": red,
    "OFFLINE": gray
}

const format_status = (status: string) => {
    return (
        <span style={{ color: status_styles[status] }}>{status}</span>
    )
}

function MojangStatus() {
    const [mojangStatus, setMojangStatus] = useState<string>("OFFLINE")

    useEffect(() => {
        const unlisten_mojang_api_event = listen<string>(
            "mojang-api-status",
            (event) => {
                setMojangStatus(event.payload);
            }
        );

        return () => {
            unlisten_mojang_api_event.then((unlisten) => unlisten());
        }
    }, []);

    return (
        <div className="mojangstatus">
            <p>Mojang: {format_status(mojangStatus)}</p>
        </div>
    );
}

interface CategoryProps {
    name: string,
    children?: React.ReactNode
}

function Category({ name, children }: CategoryProps) {
    return (
        <div className="category">
            <h3>{name}</h3>
            {children}
        </div>
    )
}

function Sidebar() {
    const [sidebarState, setSidebarState] = useState<boolean>(false);

    const style_when_hidden = {
        left: "-250px"
    };

    const style_when_visible = {
        left: "0px"
    };

    useEffect(() => {
        const unlisten_toggle_event = listen(
            "toggle-sidebar",
            () => {
                setSidebarState((state) => !state);
            }
        );

        return () => {
            unlisten_toggle_event.then((unlisten) => unlisten());
        }
    }, [])

    return (
        <div className="sidebar" style={sidebarState ? style_when_visible : style_when_hidden}>
            <Category name="STATUS">
                <MojangStatus />
            </Category>
        </div>
    );
}

export default Sidebar;