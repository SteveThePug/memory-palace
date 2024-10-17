import React, { useState, useEffect } from 'react';
import Post from './Post/Post.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { postsGet } from '../../store/posts.js'
import Modal from '../Modal.jsx'

export default function Posts() {
  const dispatch = useDispatch();
  const {posts} = useSelector((state) => state);
  
  useEffect(()=> {
    dispatch(postsGet())},[dispatch]
    )
  
  return(
    <>
      <div>
        <h1>POSTS</h1>
        {posts.map((postData, index) => (<Post postData={postData} key={index}/>))}
      </div>
    </>
  );
  }