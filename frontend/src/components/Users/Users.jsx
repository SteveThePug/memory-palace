import React, { useState, useEffect } from 'react';
import User from './User/User.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { usersGet } from '../../store/users.js';


export default function Users() {
  const dispatch = useDispatch();
  const { users } = useSelector((state) => state);
  const [showUsers, setShowUsers] = useState(false);

  useEffect(() => {
    dispatch(usersGet());
  }, [dispatch]);

  const toggleShowUsers = () => {
    setShowUsers((prevShowUsers) => !prevShowUsers);
  };

  return (
    <div>
        <div className="flex">
            <h1 className="flex-1">USERS</h1>
            <button onClick={toggleShowUsers}>
                {showUsers ? 'Hide Users' : 'Show Users'}
            </button>
        </div>
      {showUsers && users.map((userData, index) => (
        <User userData={userData} key={index} />
      ))}
    </div>
  );
}