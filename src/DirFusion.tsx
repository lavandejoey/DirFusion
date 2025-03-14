// import {useState} from "react";
// import reactLogo from "./assets/react.svg";
// import {invoke} from "@tauri-apps/api/core";
import "./DirFusion.css";
import "bootstrap/dist/css/bootstrap.min.css"

function DirFusion() {
    return (
        <main className="container-fluid row border border-primary">
            <h1>Welcome to Tauri + React</h1>

            <div className="col-4">
                <h2>Left Column</h2>
                <p>100% height</p>
            </div>

            <div className="col-8">
                <div className="row">
                    <div className="col">
                        <h2>Path 1</h2>
                        <p>100% height</p>
                    </div>
                    <div className="col">
                        <h2>Path 2</h2>
                        <p>100% height</p>
                    </div>
                </div>
                <div className="row">
                    <p>select path</p>
                </div>
            </div>
        </main>
    );
}

export default DirFusion;
