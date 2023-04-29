import React from 'react';
import axios from 'axios';

import highlighted from '../images/PH_selected.png';
import empty from '../images/PH_empty.png';
import black from '../images/PH_black.png';
import corner from '../images/PH_corner.png';
import king from '../images/PH_king.png';
import white from '../images/PH_white.png';

import {boardFromCHFEN} from '../util/boardFromCHFEN';

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

const Board = ({playerName, gameId, setGameId}) => {
    const [board, setBoard] = React.useState([[]]);
    const [selectedTiles, setSelectedTiles] = React.useState([]);
    const [selected, setSelected] = React.useState("");
    const [currentPlayer, setCurrentPlayer] = React.useState("");
    const [winner, setWinner] = React.useState("x");

    const tile_to_img = (tile, x, y) => {
        if (selectedTiles.includes(`${x}, ${y}`)) {
            return highlighted;
        }

        switch (tile) {
            case 'x':
                return corner;
                break;
            case '_':
                return empty;
                break;
            case 'w':
                return white;
                break;
            case 'k':
                return king;
                break;
            case 'b':
                return black;
                break;
            default:
                return empty;
                break;
        }
    }

    React.useEffect(() => {
        const get_board = async () => {
            let response = await axios({
                method: "POST",
                url: `${BACKEND_URL}/get_board`, 
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                data: {
                    game_id: gameId,
                },
            });

            if (response.status !== 200) {
                setGameId("");
                return;
            }

            if (response.data.fen[response.data.fen.length - 1] === 'b') {
                setCurrentPlayer('B');
            } else {
                setCurrentPlayer('W');
            }

            setBoard(boardFromCHFEN(response.data.fen));
            setWinner(response.data.winner);
        };

        get_board();
    }, []);

    const handleTileOnClick = async (x,y) => {
        if (winner !== 'x') {
            return;
        }

        setSelectedTiles([]);
        setSelected("");

        let selectedX = parseInt(selected.split(', ')[0]);
        let selectedY = parseInt(selected.split(', ')[1]);

        if (selectedTiles.includes(`${x}, ${y}`)) {
            let response = await axios({
                method: "POST",
                url: `${BACKEND_URL}/make_move`,
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                data: {
                    player_name: playerName,
                    game_id: gameId,
                    x_from: selectedX,
                    y_from: selectedY,
                    x_to: x,
                    y_to: y,
                } 
            });

            if (response.status !== 200) {
                return;
            }

            setBoard(boardFromCHFEN(response.data.fen));

            if (response.data.fen[response.data.fen.length - 1] === 'b') {
                setCurrentPlayer('B');
            } else {
                setCurrentPlayer('W');
            }

            if (response.data.winner !== winner) {
                setWinner(response.data.winner);
            }

            return;
        }
        
        if (board[x][y] === '_' || board[x][y] === 'x') {
            return;
        }
        
        if (board[x][y] === 'b' && currentPlayer === 'W') {
            return;
        }

        if ((board[x][y] === 'k' || board[x][y] === 'w') && currentPlayer === 'B') {
            return;
        }

        let response = await axios({
            method: "POST",
            url: `${BACKEND_URL}/legal_moves`, 
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json;charset=UTF-8'
            },
            data: {
                player_name: playerName,
                game_id: gameId,
                x: x,
                y: y,
            },
        });

        if (typeof response.data !== "string") {
            return;
        }
        let filtered_response = response.data.slice(2,-2).split('), (');
        setSelected(`${x}, ${y}`)
        setSelectedTiles(filtered_response);
    }

    return(
        <div>
            <div className="flex items-center justify-between text-black font-bold py-2 px-4 bg-amber-500 w-full h-[50px]">
                <a
                    className="cursor-pointer hover:text-slate-700"
                    onClick={() => {setGameId("")}}
                >
                    {"<< Abort game"}
                </a>
                <span>{playerName}</span>
            </div>

            <div className="flex justify-center w-full">
                <div
                    className="max-w-[330px] md:max-w-[660px] mx-4 my-2 grid grid-cols-11"
                >
                    {
                    board.map((row, x) => {
                        return row.map((tile, y) => 
                            <img 
                                key={`${tile}-${tile}-${x}-${y}`}
                                className="mr-0 ml-0 h-[30px] w-[30px] md:h-[60px] md:w-[60px] m-0"
                                src={tile_to_img(tile,x,y)}
                                onClick={() => handleTileOnClick(x,y)}
                            />
                        );
                    }) 
                }
                </div>
            </div>

            {
                winner === 'x' ?
                (
                    <div className="mx-4 my-2 text-center text-white font-bold">
                        Current player: {currentPlayer === 'W' ? <span>White</span> : <span className="text-black">Black</span>}
                    </div>
                ) :
                (
                    <div className="mx-4 my-2 text-center text-white font-bold">
                        {winner === 'w' ? <span>White won!</span> : <span className="text-black">Black won!</span>}
                    </div>
                )
            }
        </div>
    )

}

export default Board;

