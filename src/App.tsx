import "./App.css";
import Menus from "./components/menus/Menus";
import Sidebar from "./components/sidebar/Sidebar";
import Table from "./components/table/Table";
import Topbar from "./components/topbar/Topbar";

function App() {
  return (
    <>
      <Topbar />
      <Table />
      <Sidebar />
      <Menus />
    </>
  );
}

export default App;