var fs = require('fs')
var input = (fs.readFileSync('input.txt', 'utf8')).split('\n')
var sum = 0
input.forEach(line => {
  var operand = line.substring(0, 1)
  var num = parseInt(line.substring(1, line.length))
  if(operand === '+') {
    sum += num
  }
  else if(operand === '-') {
    sum -= num
  }
})
console.log(sum)
