import React from "react";

import Board from "./board/Board";

function App() {
    if (!localStorage.getItem("player-color")) {
        localStorage.setItem("player-color", "black");
    }

    const [playerName, setPlayerName] = React.useState(
        localStorage.getItem("player-name") || ""
    );
    const [playerColor, setPlayerColor] = React.useState(
        localStorage.getItem("player-color") || "black"
    );
    const [playerNameInput, setPlayerNameInput] = React.useState("");

    React.useEffect(() => {
       localStorage.setItem("player-name", playerName);
    }, [playerName]);

    React.useEffect(() => {
       localStorage.setItem("player-color", playerColor);
    }, [playerColor]);

    const handlePlayerNameInputChange = (e) => {
        setPlayerNameInput(e.target.value);
    };

    const handleStartGameButtonOnClick = () => {
        if (playerNameInput === "") {
            return;
        }
        setPlayerName(playerNameInput);
    };

    return (
        <div>
            {
            playerName === "" ?
                <div className="my-5 mx-[20%]">
                    <input 
                        className="bg-amber-900 border border-amber-300 text-amber-50 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
                        placeholder="Player name"
                        id="playername"
                        type="text"
                        value={playerNameInput}
                        onChange={handlePlayerNameInputChange}
                    /> 
                    <div
                        className="w-full flex justify-between mt-2"
                    >
                        <button
                            className="bg-gray-900 hover:bg-gray-800 w-[50%] mr-2 font-bold py-2 px-4 rounded"
                            onClick={() => setPlayerColor("black")}
                        >
                            Black{playerColor === "black" ? " ✓" : ""}
                        </button>
                        <button
                            className="bg-amber-200 hover:bg-amber-100 text-black w-[50%] font-bold py-2 px-4 rounded"
                            onClick={() => setPlayerColor("white")}
                        >
                            White{playerColor === "white" ? " ✓" : ""}
                        </button>
                    </div>
                    <button
                        className="bg-amber-500 hover:bg-amber-700 w-full mt-2 font-bold py-2 px-4 rounded"
                        onClick={handleStartGameButtonOnClick}
                    >
                        Start game
                    </button>
                </div>
                :
                <Board 
                    playerName={playerName}
                    setPlayerName={setPlayerName}
                /> 
            }
        </div>
    );
}

export default App;
