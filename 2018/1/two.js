var fs = require('fs')
var input = (fs.readFileSync('input.txt', 'utf8')).split('\n')
var sum = 0
var dictionary = {
    0: true
}
var problemFinished = false

while(!problemFinished) {
    readInput()
}

function readInput() {
    input.forEach(line => {
        var i = 1
        if(problemFinished) {
            return
        }
        var operand = line.substring(0, 1)
        var num = parseInt(line.substring(1, line.length))
        if(operand === '+') {
          sum += num
        }
        else if(operand === '-') {
          sum -= num
        }
        if(dictionary[sum] != undefined) {
            console.log(`First repeat value: ${sum}`)
            problemFinished = true
        }
        dictionary[sum] = true
      })
    return sum
}
