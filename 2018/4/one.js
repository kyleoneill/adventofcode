var fs = require('fs')

function main() {
    var input = (fs.readFileSync('./4/input.txt', 'utf8')).split('\n')
    input = sortByDateTime(input)
    var guardArray = createGuards(input)
    var consistentGuard = getConsistentGuard(guardArray)
    var sleepiestGuard = getSleepiestGuard(guardArray)
    console.log(`ID: ${sleepiestGuard.id}\nMinute: ${sleepiestGuard.sleepiestMinute}\nPt2 ID: ${consistentGuard.id}\nPt2 Minute: ${consistentGuard.sleepiestMinute}`)
}

function sortByDateTime(data) {
    //var year = entry.substring(1, 5)
    //var month = entry.substring(6, 8)
    //var day = entry.substring(9, 11)
    //var hour = entry.substring(12, 14)
    //var minute = entry.substring(15, 17)
    data.sort(function(a, b){
        if(a.substring(6, 8) == b.substring(6, 8)) {
            if(a.substring(9, 11) == b.substring(9, 11)) {
                if(a.substring(12, 14) == b.substring(12, 14)) {
                    return parseInt(a.substring(15, 17)) - parseInt(b.substring(15, 17))
                }
                else {
                    return parseInt(a.substring(12, 14)) - parseInt(b.substring(12, 14))
                }
            }
            else {
                return parseInt(a.substring(9, 11)) - parseInt(b.substring(9, 11))
            }
        }
        else {
            return parseInt(a.substring(6, 8)) - parseInt(b.substring(6, 8))
        }
    })
    return data
}

function createGuards(schedule) {
    let guardArray = []
    let currentGuard
    let minutesAsleep
    let sleepyTimeStart
    for(let i = 0; i < schedule.length; i++) {
        let year = schedule[i].substring(1, 5)
        let month = schedule[i].substring(6, 8)
        let day = schedule[i].substring(9, 11)
        let hour = schedule[i].substring(12, 14)
        let minute = schedule[i].substring(15, 17)
        let splitSchedule = schedule[i].split(" ")
        let guardID = splitSchedule[3]
        let scheduleAction = splitSchedule[2]
        //If the current action is a shift change
        if(scheduleAction == "Guard") {
            //If there is currently a guard on shift
            if(currentGuard != undefined) {
                //Add the minutes spent asleep and save the guard in the array
                currentGuard.timeAsleep += minutesAsleep
                for(let j = 0; j < guardArray.length; j++) {
                    if(guardArray[j].id == currentGuard.id) {
                        guardArray[j] = currentGuard
                        break
                    }
                }
            }
            //Check if the new guard exists
            var guardExists = false
            var k
            for(k = 0; k < guardArray.length; k++) {
                if(guardArray[k].id == guardID) {
                    guardExists = true
                    break
                }
            }
            //If so, current guard is set to the stored guard
            if(guardExists) {
                currentGuard = guardArray[k]
            }
            //Else, current guard is set to a newly created guard and the new guard is added to the array
            else {
                currentGuard = new Guard(guardID, 0, [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 0)
                guardArray.push(currentGuard)
            }
            minutesAsleep = 0
        }
        //If the current action is a guard falling asleep
        else if(scheduleAction == "falls") {
            sleepyTimeStart = new Date(year, month, day, hour, minute)
        }
        else if(scheduleAction == "wakes") {
            let sleepyTimeStop = new Date(year, month, day, hour, minute)
            let timeSlept = sleepyTimeStop.getMinutes() - sleepyTimeStart.getMinutes()
            minutesAsleep += timeSlept
            for(let l = 0; l < timeSlept; l++) {
                currentGuard.sleepyMinutes[l + sleepyTimeStart.getMinutes()] += 1
            }
            sleepyTimeStart = null
        }
    }
    return guardArray
}

function getSleepiestGuard(guardArray) {
    var sleepiestGuard = guardArray[0]
    var sleepAtSleepiestMinute = 0
    for(var i = 0; i < guardArray.length; i++) {
        if(guardArray[i].timeAsleep > sleepiestGuard.timeAsleep) {
            sleepiestGuard = guardArray[i]
        }
    }
    for(var k = 0; k < sleepiestGuard.sleepyMinutes.length; k++) {
        if(sleepiestGuard.sleepyMinutes[k] > sleepAtSleepiestMinute) {
            sleepAtSleepiestMinute = sleepiestGuard.sleepyMinutes[k]
            sleepiestGuard.sleepiestMinute = k
        }
    }
    return sleepiestGuard
}

function getConsistentGuard(guardArray) {
    var mostCommonMinute = 0
    var sleepAtMostCommonMinute = 0
    var mostCommonMinuteGuard
    for(var i = 0; i < guardArray.length; i++) {
        var currentGuard = guardArray[i]
        var sleepAtSleepiestMinute = 0
        for(var j = 0; j < currentGuard.sleepyMinutes.length; j++) {
            if(currentGuard.sleepyMinutes[j] > sleepAtSleepiestMinute) {
                sleepAtSleepiestMinute = currentGuard.sleepyMinutes[j]
                currentGuard.sleepiestMinute = j
            }
        }
        if(sleepAtSleepiestMinute > sleepAtMostCommonMinute) {
            mostCommonMinute = currentGuard.sleepiestMinute
            sleepAtMostCommonMinute = sleepAtSleepiestMinute
            mostCommonMinuteGuard = currentGuard
        }
    }
    return mostCommonMinuteGuard
}

function Guard(id, timeAsleep, sleepyMinutes, sleepiestMinute) {
    this.id = id
    this.timeAsleep = timeAsleep
    this.sleepyMinutes = sleepyMinutes
    this.sleepiestMinute = sleepiestMinute
}

main()
