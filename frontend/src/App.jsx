import {useEffect} from 'react'
import { useDispatch } from 'react-redux';

import {
  Route, Routes
} from "react-router-dom";

import Home from "./pages/Home";
import Navbar from './components/Navbar/Navbar.jsx'

import { userSet } from './store/user.js'

export default function App() {
  const dispatch = useDispatch();
  
  useEffect(() => {
    const token = JSON.parse(localStorage.getItem('token'));
    if (token) dispatch(userSet(token));
  }, [dispatch]);
  
  return (
    <>
      <Navbar/>
      <Routes>
        <Route index path="/" element={<Home/>}/>
      </Routes>
    </>
  );
}