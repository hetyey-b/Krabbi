import React from "react";
import axios from 'axios';

import Board from "./board/Board";

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

function App() {
    const [playerWhiteBot, setPlayerWhiteBot] = React.useState(false);
    const [playerBlackBot, setPlayerBlackBot] = React.useState(false);
    const [playerName, setPlayerName] = React.useState(localStorage.getItem("playerName") || "");

    const handleStartGameOnClick = async () => {
        if (!playerName) {
            return;
        }

        localStorage.setItem("playerName", playerName);
        
        try {
            let response = await axios({
                method: "POST",
                url: `${BACKEND_URL}/new_game`, 
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                data: {
                    player_name: playerName,
                    bot_white: playerWhiteBot,
                    bot_black: playerWhiteBot,
                },
            });

            localStorage.setItem("currentGameId", response.data);
        } catch (err) {
            console.error(err);
        }
    }

    const handlePlayerNameInputOnChange = (e) => {
        setPlayerName(e.target.value);
    }

    return (
        <div className="mx-4 my-2 grid grid-cols-2 gap-4">
            <div className="col-span-2">
                <label htmlFor="playerName" className="block mb-2 text-sm font-medium text-white">Name</label>
                <input 
                    type="text" 
                    id="playerName" 
                    className="bg-amber-900 border border-amber-500 text-white text-sm rounded-lg focus:ring-amber-500 focus:border-amber-500 block w-full p-2.5"
                    placeholder="Urist" 
                    value={playerName}
                    onChange={handlePlayerNameInputOnChange}
                />
            </div>
            <button 
                className="bg-slate-300 hover:bg-slate-100 text-black font-bold py-2 px-4 rounded"
                onClick={() => {setPlayerWhiteBot(true)}}
            >
                Bot{playerWhiteBot ? " ✓" : ""}
            </button>
            <button 
                className="bg-slate-900 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded"
                onClick={() => {setPlayerBlackBot(true)}}
            >
                Bot{playerBlackBot ? " ✓" : ""}
            </button>
            <button 
                className="bg-slate-300 hover:bg-slate-100 text-black font-bold py-2 px-4 rounded"
                onClick={() => {setPlayerWhiteBot(false)}}
            >
                Human{playerWhiteBot ? "" : " ✓"}
            </button>
            <button 
                className="bg-slate-900 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded"
                onClick={() => {setPlayerBlackBot(false)}}
            >
                Human{playerBlackBot ? "" : " ✓"}
            </button>
            <button 
                className="col-span-2 bg-amber-500 hover:bg-amber-400 text-black font-bold py-2 px-4 rounded"
                onClick={handleStartGameOnClick}
            >
                Start
            </button>
        </div>
    );
}

export default App;
