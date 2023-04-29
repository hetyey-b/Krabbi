import React from "react";
import axios from 'axios';

import Board from "./board/Board";

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

function App() {
    const [serverSanityCheckError, setServerSanityCheckError] = React.useState('');
    const [playerWhiteBot, setPlayerWhiteBot] = React.useState(false);
    const [playerBlackBot, setPlayerBlackBot] = React.useState(false);
    const [playerName, setPlayerName] = React.useState(localStorage.getItem("playerName") || "");
    const [gameId, setGameId] = React.useState(localStorage.getItem("gameId") || "");

    React.useEffect(() => {
        const sanityCheck = async () => {
            try {
                let response = await axios({
                    method: "GET",
                    url: `${BACKEND_URL}/`, 
                });
            } catch(err) {
                localStorage.setItem("gameId", "");
                setGameId("");
                setServerSanityCheckError(err);
            }
        }

        sanityCheck();
    }, []);

    const handleStartGameOnClick = async () => {
        if (!playerName) {
            return;
        }

        if (playerWhiteBot && playerBlackBot) {
            alert("Cannot create a game without a human player!");
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
                    bot_black: playerBlackBot,
                },
            });

            setGameId(response.data);
        } catch (err) {
            console.error(err);
        }
    }

    React.useEffect(() => {
        if (!gameId) {
            return;
        }
        localStorage.setItem("gameId", gameId);
    }, [gameId])

    const handlePlayerNameInputOnChange = (e) => {
        setPlayerName(e.target.value);
    }

    if (gameId) {
        return (
            <Board
                playerName={playerName}
                gameId={gameId}
                setGameId={setGameId}
            />
        )
    }

    if (serverSanityCheckError !== '') {
        return (
            <div className="mx-4 my-4 items-center text-center md:px-[33%]">
                <p
                    className="w-full font-bold"
                >
                    Error: Server connection not found
                </p>
                <p
                    className="w-full"
                >
                    Refresh the page, and try again later
                </p>
            </div>
        )
    }

    return (
        <div className="mx-4 my-2 grid grid-cols-2 gap-4 md:px-[33%]">
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
                onClick={() => {
                    if (playerBlackBot) {
                        setPlayerBlackBot(false);
                    }
                    setPlayerWhiteBot(true);
                }}
            >
                Bot{playerWhiteBot ? " ✓" : ""}
            </button>
            <button 
                className="bg-slate-900 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded"
                onClick={() => {
                    if (playerWhiteBot) {
                        setPlayerWhiteBot(false);
                    }
                    setPlayerBlackBot(true);
                }}
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

            <footer>
                Wooden texture from: <a href="https://www.freepik.com/free-photo/wooden-wood-backgrounds-textured-pattern-wallpaper-concept_2760885.htm#query=wood%20texture&position=8&from_view=keyword&track=robertav1_2_sidr">Image by rawpixel.com</a> on Freepik
            </footer>
        </div>
    );
}

export default App;
