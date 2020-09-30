const fs = require('fs'); 

let map = [];

let states = {
    empty: 0,
    red: 1,
    green: 2,
    blue: 3,
    cyan: 4
}

//neighbors: clockwise, color: 0 - empty, >0 - rgb
//let tile = {neighbors: [1,2,3,4,5], state: 0}

map[0] = {neighbors: [1,2,3,4,5],      state: 0}
map[1] = {neighbors: [0,6,7,8,9],      state: 0}
map[2] = {neighbors: [0,9,10,11,12],   state: states.red}
map[3] = {neighbors: [0,12,13,14,15],  state: states.green}
map[4] = {neighbors: [0,15,16,17,18],  state: states.blue}
map[5] = {neighbors: [0,18,19,20,6],   state: states.cyan}

map[6] = {neighbors: [1,5,-1,-1,-1],   state: 0}

map[7] = {neighbors: [1,-1,-1,-1,-1],  state: 0}
map[8] = {neighbors: [1,-1,-1,-1,-1],  state: 0}
map[9] = {neighbors: [2,1,-1,-1,-1],   state: 0}

map[10] = {neighbors: [2,-1,-1,-1,-1], state: 0}
map[11] = {neighbors: [2,-1,-1,-1,-1], state: 0}
map[12] = {neighbors: [3,2,-1,-1,-1],  state: 0}

map[13] = {neighbors: [3,-1,-1,-1,-1], state: 0}
map[14] = {neighbors: [3,-1,-1,-1,-1], state: 0}
map[15] = {neighbors: [4,3,-1,-1,-1],  state: 0}

map[16] = {neighbors: [4,-1,-1,-1,-1], state: 0}
map[17] = {neighbors: [4,-1,-1,-1,-1], state: 0}
map[18] = {neighbors: [5,4,-1,-1,-1],  state: 0}

map[19] = {neighbors: [5,-1,-1,-1,-1], state: 0}
map[20] = {neighbors: [5,-1,-1,-1,-1], state: 0}

const jsonContent = JSON.stringify(map); 

fs.writeFile("map.json", jsonContent, 'utf8', function (err) {
    if (err) {
        console.log("An error occured while writing JSON Object to File.");
        return console.log(err);
    }
 
    console.log("JSON file has been saved.");
});
 