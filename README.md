Creating the Todo list application using Rust

Terminal Todo list App

The todo list minimal feature set includes:

Add, remove, edit todos
Mark todos as "done"
Save and load todos

**Commands we are going to use in this project

>> todo add  "taskname" "durationtime" "starttime" // the duration time can be an uint type  and start time will be string type

>> todo delete  "taskname"  // to delete a task simply

>> todo edit  "taskname" "new-duration-time" "old-duration-time" // if you want to keep duration time as same as previous you can keep zero

>> todo completed  "taskname" // the task is completed

>> todo get completed-tasks // it will returns all the completed tasks

>> todo get current-tasks // it will return all the present tasks

>> tdo get deleted-tasks // it will return all the deleted tasks

>> todo exit // it will exit the app

***Note : There is no need to keep "" this in between the tasknames and so on


todo Contains {
    taskName,
    duration time,
    end time
}
