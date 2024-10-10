import React, { useState, useEffect } from 'react';
import Post from './Post/Post.jsx';
import Form from './Form.jsx'
import { useSelector, useDispatch } from 'react-redux';
import { postsGet } from '../../store/posts.js'
import Modal from '../Modal.jsx'

export default function Posts() {
  const dispatch = useDispatch();
  const {posts, user} = useSelector((state) => state);
  
  const [isFormModalOpen, setIsFormModalOpen] = useState(false);
  const openFormModal = () => setIsFormModalOpen(true);
  const closeFormModal = () => setIsFormModalOpen(false);
  
  useEffect(()=> {
    dispatch(postsGet())},[dispatch]
    )
  
  return(
    <>
      <Modal isOpen={isFormModalOpen} onClose={closeFormModal}>
        <Form />
      </Modal>
      
      {user && <button onClick={openFormModal}>Upload</button>}
      
      <div className='border-2 p-2'>
        {posts.map((postData, index) => (<Post postData={postData} key={index}/>))}
      </div>
    </>
  );
  }