import React from 'react';
import axios from 'axios';

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

const Board = ({playerName, setPlayerName}) => {
    const [tiles, setTiles] = React.useState([[]]);

    React.useEffect(() => {

    }, []);

    const handleAbortOnClick = () => {
        setPlayerName("");
    };

    return(
        <div>
            <div
                className="flex justify-between bg-sky-700 px-4 py-2"
            >
                <a
                    onClick={handleAbortOnClick}
                >
                    {"<< Abort game"}
                </a>
                <span className="font-bold">{playerName}</span>
            </div>
            <div className="text-center mt-2">
                Insert Board Here
            </div>
        </div>
    )

}

export default Board;

