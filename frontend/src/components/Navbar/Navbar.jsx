import React, { useState, useEffect } from 'react';
import { NavLink } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { userLogOut, userSet } from '../../store/user.js';
import Login from './Login.jsx';
import Modal from '../Modal.jsx'
import Form from './Form.jsx'

export default function Navbar() {
  const user = useSelector((state) => state.user?.user);
  const dispatch = useDispatch();

  // Login Modal
  const [isLoginModalOpen, setIsLoginModalOpen] = useState(false);
  const openLoginModal = () => setIsLoginModalOpen(true);
  const closeLoginModal = () => setIsLoginModalOpen(false);
  // Post Modal
  const [isPostModalOpen, setIsPostModalOpen] = useState(false);
  const openPostModal = () => setIsPostModalOpen(true);
  const closePostModal = () => setIsPostModalOpen(false);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) dispatch(userSet(JSON.parse(token)));
  }, [dispatch]);

  const logout = () => dispatch(userLogOut());

  return (
    <div className='flex justify-between p-4'>
      {/* Logo/Title */}
      <NavLink to="/">Home</NavLink>
      {/* User Info */}
      {user && (
        <>
          <button onClick={openPostModal}>Upload</button>
          <button onClick={logout}>Logout</button>
          <NavLink>{user.username}</NavLink>
        </>
      )}
      {/* Login */}
      {!user && (
        <>
          <button onClick={openLoginModal}>LOGIN</button>
        </>
      )}

      {/* Login Modal */}
      <Modal isOpen={isLoginModalOpen} onClose={closeLoginModal}>
        <Login onLoginSuccess={closeLoginModal} />
      </Modal>

      {/* Post Modal */}
      <Modal isOpen={isPostModalOpen} onClose={closePostModal}>
        <Form onFormSuccess={closePostModal}/>
      </Modal>

    </div>

  );
}