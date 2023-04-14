import React from 'react';
import axios from 'axios';

import empty from '../images/PH_empty.png';
import black from '../images/PH_black.png';
import corner from '../images/PH_corner.png';
import king from '../images/PH_king.png';
import white from '../images/PH_white.png';

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

const tile_to_img = (tile) => {
    switch (tile) {
        case 'X':
            return corner;
            break;
        case '.':
            return empty;
            break;
        case 'T':
            return corner;
            break;
        case 'W':
            return white;
            break;
        case 'K':
            return king;
            break;
        case 'B':
            return black;
            break;
        default:
            return empty;
            break;
    }
}

const Board = ({playerName, setPlayerName}) => {
    return(
        <div>
        </div>
    )

}

export default Board;

