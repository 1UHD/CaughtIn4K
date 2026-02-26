import { useState } from "react";
import "./inputbox.css";

interface InputBoxProps {
    placeholder?: string;
    read_only?: boolean;
    on_key_down: (param: any) => void;
    privacy_box?: boolean;
}

function InputBox({ placeholder = "", on_key_down, privacy_box = false }: InputBoxProps) {
    const [privacyState, setPrivacyState] = useState<boolean>(true)

    const on_toggle_privacy = () => {
        setPrivacyState((state) => !state);
    }

    const on_key_event = (event: any) => {
        on_key_down(event);
    }

    return (
        <div className="input-box">
            {privacy_box ? <p id="privacy-state" onClick={on_toggle_privacy}>{privacyState ? "Show" : "Hide"}</p> : null}
            <input
                type="text"
                onKeyDown={on_key_event}
                placeholder={placeholder}
                style={{
                    filter: privacyState ? "blur(5px)" : "none",
                    userSelect: privacyState ? "none" : "auto"
                }}
            />
        </div>
    )
}

export default InputBox;