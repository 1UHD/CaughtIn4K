import "./buttons.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
const appWindow = getCurrentWindow();

function Buttons() {
    const close = () => {
        appWindow.close();
    }

    const minimize = () => {
        appWindow.minimize();
    }

    return (
        <div className="buttons">
            <div className="minimize" onClick={minimize}>
                <img src="minimize/active.png" id="mactive" />
                <img src="minimize/inactive.png" id="minactive" />
                <img src="minimize/pressed.png" id="mpressed" />
            </div>
            <div className="close" onClick={close}>
                <img src="close/active.png" id="cactive" />
                <img src="close/inactive.png" id="cinactive" />
                <img src="close/pressed.png" id="cpressed" />
            </div>
        </div>
    );
}

export default Buttons;