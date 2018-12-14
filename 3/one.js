var fs = require('fs')
var input = (fs.readFileSync('./3/input.txt', 'utf8')).split('\n')

var overlapInches = 0
var fabricSheet = {}

function main() {
    //draw the fabric sheet
    input.forEach(line => {
        claim = new Object
        var splitSpace = line.split(' ')
        claim.id = splitSpace[0].replace('#', '')
        claim.position = splitSpace[2].replace(':', '')
        claim.size = splitSpace[3]
        draw(claim)
    })
    for (line in fabricSheet) {
        if(fabricSheet[line].indexOf("x") > -1) {
            overlapInches++
        }
    }
    console.log(`Answer: ${overlapInches}`)
}

function draw(claim) {
    var positions = claim.position.split(',')
    var size = claim.size.split('x')
    //var squareSizage = size[0]*size[1]
    for(var i = 0; i < parseInt(size[0]); i++) {
        for(var j = 0; j < parseInt(size[1]); j++) {
            var xPos = parseInt(positions[0]) + i
            var yPos = parseInt(positions[1]) + j
            var coord = xPos.toString()+","+yPos.toString()
            if(fabricSheet[coord] == undefined) {
                fabricSheet[coord] = claim.id
            }
            else {
                fabricSheet[coord] = "x"
            }
        }
    }
}

main()
