import React, {Component } from "react";
import axios from "axios";

class ToDoItem extends Component {
    state = {
        "title": this.props.title,
        "status": this.props.status,
        "button": this.props.processStatus(this.props.status),
    }

    processStatus(status) {
        if (status === "PENDING") {
            return "edit"
        } else {
            return "delete"
        }
    }
    
    inverseStatus(status) {
        if (status === "PENDING") {
            return "DONE"
        } else {
            return "PENDING"
        }
    }

    sendRequest = () => {
        axios.post("http://127.0.0.1:8080/v1/item/"+this.state.button,
            {
                "title": this.state.title,
                "status": this.state.status,
            },
            {
                "headers": {
                    "user-token": "React ToDoItem token"
                }
        })
        .then(response => {
            this.props.passBackResponse(response)
        })
    }

    render() {
        return(
            <div>
                <p>{this.state.title}</p>
                <button onClick={this.sendRequest}>{this.state.button}</button>
            </div>
        )
    }
}

export default ToDoItem