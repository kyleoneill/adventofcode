var fs = require('fs')
var input = (fs.readFileSync('./2/input.txt', 'utf8')).split('\n')
var countOfTwos = 0
var countOfThrees = 0

function main() {
    input.forEach(line => {
        checkForRepeat(line)
    })
    console.log(`Answer: ${countOfThrees*countOfTwos}`)
}

function checkForRepeat(string) {
    var twoDone = false
    var threeDone = false
    var uniqueChars = [0]
    for(var i = 0; i < string.length; i++){
        if(!(uniqueChars.indexOf(string[i]) > -1)) {
            uniqueChars.push(string[i])
        }
    }
    uniqueChars.splice(0, 1)
    uniqueChars.forEach(character => {
        var regex = new RegExp(character, "g")
        var count = (string.match(regex) || []).length
        if(count == 2 && !twoDone) {
            countOfTwos++
            twoDone = true
        }
        else if(count == 3 && !threeDone) {
            countOfThrees++
            threeDone = true
        }
    })
}

main()
