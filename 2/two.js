var fs = require('fs')
var input = (fs.readFileSync('./2/input.txt', 'utf8')).split('\n')

function main() {
    var searchComplete
    for(var i = 0; i < input.length; i++) {
        for(var j = i + 1; j < input.length; j++) {
            searchComplete = compareString(input[i], input[j])
            if(searchComplete) {
                console.log(`Line One: ${input[i]}\nLine Two: ${input[j]}`)
                return
            }
        }
    }
}

main()


function compareString(one, two) {
    var incorrectChars = 0
    for(var i = 0; i < one.length; i++) {
        if(one[i] != two[i]) {
            incorrectChars++
        }
        if(incorrectChars > 1) {
            return false
        }
    }
    if(incorrectChars == 1) {
        return true
    }
    else{
        return false
    }
}
