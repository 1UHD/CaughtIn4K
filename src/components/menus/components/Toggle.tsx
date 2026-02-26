import { useState } from "react";
import "./toggle.css";
import { dark_green, red } from "../../../functional/statsFormatter";

interface ToggleProps {
    default_state: boolean;
    event: (param: boolean) => void;
}

function Toggle({ default_state, event }: ToggleProps) {
    const [toggleState, setToggleState] = useState<boolean>(default_state);

    const on_toggle_toggle = () => {
        event(!toggleState);
        setToggleState((state) => !state);
    };

    const off_style = {
        left: "0px",
        backgroundColor: red
    };

    const on_style = {
        left: "26px",
        backgroundColor: dark_green
    };

    return (
        <div className="toggle" onClick={on_toggle_toggle}>
            <div className="toggle-slider" />
            <div className="toggle-knob" style={toggleState ? on_style : off_style}/>
        </div>
    );
}

export default Toggle;