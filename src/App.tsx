import "./App.css";
import Sidebar from "./components/sidebar/Sidebar";
import Table from "./components/table/Table";
import Topbar from "./components/topbar/Topbar";

function App() {
  return (
    <>
      <Topbar />
      <Table />
      <Sidebar />
    </>
  );
}

export default App;