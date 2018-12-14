var fs = require('fs')
var input = (fs.readFileSync('./3/input.txt', 'utf8')).split('\n')

var overlapInches = 0
var fabricSheet = {}
var claimIDs = []
var poisonedClaims = []
var overlapFreeID

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
    //check fabric sheet
    for(inch in fabricSheet) {
        if(fabricSheet[inch].indexOf("x") > -1) {
            overlapInches++
        }
    }
    //find overlap free claim
    for(claimID in claimIDs) {
        if(poisonedClaims.indexOf(claimIDs[claimID]) == -1) {
            overlapFreeID = claimIDs[claimID]
            break
        }
    }
    // 1273 too high
    //26 wrong
    console.log(`Inches of Overlap: ${overlapInches}\nOverlap Free Claim ID: ${overlapFreeID}`)
}

function draw(claim) {
    var positions = claim.position.split(',')
    var size = claim.size.split('x')
    var isPoisoned = false
    claimIDs.push(claim.id)
    for(var i = 0; i < parseInt(size[0]); i++) {
        for(var j = 0; j < parseInt(size[1]); j++) {
            var xPos = parseInt(positions[0]) + i
            var yPos = parseInt(positions[1]) + j
            var coord = xPos.toString()+","+yPos.toString()
            if(fabricSheet[coord] == undefined) {
                fabricSheet[coord] = claim.id
            }
            else {
                if(fabricSheet[coord] != "x" && poisonedClaims.indexOf(fabricSheet[coord]) == -1) {
                    poisonedClaims.push(fabricSheet[coord])
                }
                if(!isPoisoned) {
                    poisonedClaims.push(claim.id)
                }
                fabricSheet[coord] = "x"
                isPoisoned = true
            }
        }
    }
}

main()
