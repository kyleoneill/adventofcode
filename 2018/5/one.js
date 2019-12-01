var fs = require('fs')

function main() {
    var input = (fs.readFileSync('./5/input.txt', 'utf8')).split('\n')
    var polymer = new polymerTracker(input[0], 0)
    var reactionDone = false
    while(!reactionDone) {
        polymer = react(polymer)
        if(polymer.numOfReactions == 0) {
            reactionDone = true
        }
    }
    console.log(polymer)
    console.log(`Remaining polymer units: ${polymer.polymer.length}`)
}

function react(polymerObj) {
    //var end = polymerLocation + 2 //substring is inclusive of the start but not the end
    var numberOfReactions = 0
    var polymer = polymerObj.polymer
    for(var i = 0; i < polymer.length; i++) {
        var sub = polymer.substring(i, i+2)
        var firstLetter = sub.substring(0,1)
        var secondLetter = sub.substring(1)
        var oneCapital = XOR(firstLetter == firstLetter.toUpperCase(), secondLetter == secondLetter.toUpperCase())
        var sameLetters = firstLetter.toUpperCase() == secondLetter.toUpperCase()
        if(oneCapital && sameLetters) {
            numberOfReactions++
            polymerNewFirstHalf = polymer.substring(0, i)
            polymerNewSecondHalf = polymer.substring(i+2, polymer.length)
            polymer = polymerNewFirstHalf + polymerNewSecondHalf
            i = i--
        }
    }
    polymerObj.polymer = polymer
    polymerObj.numOfReactions = numberOfReactions
    return polymerObj
}

function XOR(a,b) {
    return ( a || b ) && !( a && b );
}

function polymerTracker(polymer, numOfReactions) {
    this.polymer = polymer
    this.numOfReactions = numOfReactions
}

main()