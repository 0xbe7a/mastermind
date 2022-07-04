import React from 'react';

type MenuProps = {
  onFindGuess: () => void,
  onSubmit: () => void,
  onReset: () => void,
  canSubmit: boolean,
  canReset: boolean,
  canFindOptimal: boolean,
  onPegCountChange: (count: number) => void,
  onColorCountChange: (count: number) => void,
  pegCount: number,
  colorCount: number,
  maxPegs: number,
  maxColors: number,
};

export const Menu = (props: MenuProps) => {
  return (
    <>
    <div className='menu'>
      <Settings 
      {...props}
      />
    </div>
    <div className='menu'>
      <MenuButton icon="🔍" text="Find optimal Guess" className="blue" disabled={!props.canFindOptimal} onClick={props.onFindGuess} />
      <MenuButton icon="+" text="Submit" className="green" disabled={!props.canSubmit} onClick={props.onSubmit} />
      <MenuButton icon="↺" text="Reset" className="red" disabled={!props.canReset} onClick={props.onReset} />
    </div>
    
    </>

  )
}
type MenuButtonProps = {
  onClick: () => void,
  icon: string,
  text: string,
  className: string,
  disabled?: boolean
}

export const MenuButton = (props: MenuButtonProps) => {
  return (
    <button {...props} >
      <span>{props.icon}</span> {props.text}
    </button>
  )
}

type SettingsProps = {
  onPegCountChange: (count: number) => void,
  onColorCountChange: (count: number) => void,
  pegCount: number,
  colorCount: number,
  maxPegs: number,
  maxColors: number
}

export const Settings = (props: SettingsProps) => {
  return (
    <>
      <div className='row'><span>Peg Count: </span> <input type="number" value={props.pegCount} onChange={(e) => props.onPegCountChange(parseInt(e.target.value))} min={1} max={props.maxPegs} /></div>
      <div className='row'><span>Color Count: </span> <input type="number" value={props.colorCount} onChange={(e) => props.onColorCountChange(parseInt(e.target.value))} min={2} max={props.maxColors} /></div>
    </>
  )
}