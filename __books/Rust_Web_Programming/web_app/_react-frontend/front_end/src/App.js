// import logo from './logo.svg';
// import './App.css';

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.js</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }

// export default App;

import React, { Component } from 'react';
import axios from 'axios';

import ToDoItem from './components/ToDoItem';
import CreateToDoItem from './components/CreateToDoItem';

class App extends Component {
  state = {
    "pending_items": [],
    "done_items": [],
    "pending_count": 0,
    "done_count": 0,
  }

  getItems() {
    axios.get(
      "http://127.0.0.1:8080/v1/item/get",
      {
        headers: {
          "token": "Axios Token"
        }
      }
    )
    .then(response => {
      let pending_items = response.data["pending_items"]
      let done_items = response.data["done_items"]
      let pending_count = response.data["pending_count"]
      let done_count = response.data["done_count"]
      this.setState({
        "pending_items": this.processItemValues(pending_items),
        "done_items": this.processItemValues(done_items),
        "pending_count": pending_count,
        "done_count": done_count,
      })
    })
  }

  componentDidMount() {
    this.getItems()
  }

  processItemValues(items) {
    let itemList = []
    items.forEach((item, _) => {
      itemList.push(
        <ToDoItem key={item.title + item.status}
                  title={item.title}
                  status={item.status.status}
                  passBackResponse={this.handleReturnState}/>
      )
    })
    return itemList
  }

  handleReturnedState = (response) => {
      let pending_items = response.data["pending_items"]
      let done_items = response.data["done_items"]
      this.setState({
          "pending_items": 
           this.processItemValues(pending_items),
          "done_items": this.processItemValues(done_items),
          "pending_count": 
           response.data["pending_count"],
          "done_count": response.data["done_count"]
      })
  }

  render() {
    return (
      <div className='App'>
        <h1>Pending Items</h1>
        <p>pending item count: {this.state.pending_count}</p>
        {this.state.pending_items}

        <h1>Done Items</h1>
        <p>done item count: {this.state.done_count}</p>
        {this.state.done_items}
      </div>
    )
  }
}

export default App;