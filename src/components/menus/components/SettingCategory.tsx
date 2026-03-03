import "./settingcategory.css";

interface CategoryProps {
    name: string,
    children?: React.ReactNode
}

function SettingsCategory({ name, children }: CategoryProps) {
    return (
        <div className="settings-category" id={name}>
            <p id="settings-category-name">{name}</p>
            {children}
        </div>
    )
}

export default SettingsCategory;