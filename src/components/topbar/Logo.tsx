import "./logo.css";

function Logo() {
    return (
        <div className="logo">
            <img src="logo/logo.png" id="logo"/>
            <img data-tauri-drag-region src="logo/title.png" id="title"/>
        </div>
    );
}

export default Logo;