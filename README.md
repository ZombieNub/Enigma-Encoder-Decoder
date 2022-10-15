# Enigma-Encoder-Decoder
A Basic Enigma Encoder alongside an Enigma Decoder

Note that I haven't actually made either the encoder nor decoder yet, so sit tight, I suppose.

# Goal
The Enigma machine was a cryptographic device in which the encoding of messages was dependent on the state of the machine.
It was used by Germany in WW2 to secure communications and prevent decryption from the Allies. Despite this, the Allies
managed to crack the Enigma code, and the machine was rendered useless.
In this project, I will attempt to recreate the Enigma machine, and use it to encode and decode messages.
I will then create a program that will attempt to crack the code, and decode the message.
The two tasks are quite different, and the encoder will be the far easier of the two.
But the team at Bletchley Park managed to crack the code, so supposedly any nerd could do it.

# How the Enigma Machine Works
This will be a theoretical implementation of the Enigma machine. I will not be using any rea l-world components, nor will
I make any attempt to replicate the physical machine.

Input into the Enigma is done with a keyboard. Here, we will use a string of characters.

Output is done with a screen. Here, we will also use a string of characters.

The Enigma machine has three components: the plugboard, the rotors, and the reflector.
1. The plugboard simply swaps two letters. For example, if the plugboard swaps A and B, then the input A will be sent as B. Likewise, the input B will be sent as A. The same happens in reverse: If the output is A, then it will be sent as B, and vice versa.
2. The rotors were a set of gears that could be inserted into the machine. They are mappings of letters to other letters. However, they are not static mappings. Instead, they are mappings that are dependent on the state of the machine. The rotors are also not static. They can be rotated, and this changes the mapping.
3. The reflector is a set of wires that reflect the input. For example, if the reflector swaps A and B, then the input A will be sent as B, and the input B will be sent as A. More importantly, the reflector sends its input signal back through the rotors, and then back through the plugboard. This is how the Enigma machine is able to send back its signal.

The important thing to note is that the rotors are not static. They can be rotated, and this changes the mapping. This is how the Enigma machine is able to map the same character to different characters, or different characters to the same character, without causing the message to be undecodable.
This made the Germans believe that the Enigma machine was unbreakable, as the rotors were constantly changing. Unlike a
traditional ceasar cipher, the Enigma machine was not a static mapping. The rotors, and therefore the mapping, changed with each character.

However, there was a critical flaw: The reflector. The reflector never returned the same letter it received. This opened up a vulnerability in the Enigma machine, and the Allies were able to exploit it.

But that's for future me to figure out. Right now, I just need to make the encoder and decoder.
The design of the Enigma means that, by designing an encoder, we also design the decoder. This is because the encoder and decoder are the same machine.

# Implementation
While it's tempting to map characters to characters, it's actually easier to map integers to integers.
This is because the rotors and reflector need to account for a constantly shifting position, and this is easier to do with integers.
Instead of mapping characters to characters directly, we can instead use arithmetic to directly map integers to integers.
To convert the numbers back to characters, we can just invert the mapping.

## Plugboard
The plugboard can be a simple dictionary of integers to integers. The keys are the input, and the values are the output. Not too difficult.
Importantly, the plugboard can be user-defined.

## Rotors
The rotors are also dictionaries that map integers to integers.

## Reflector
Reflector is implemented the same way we implement the plugboard, but with a baked-in dictionary rather than a user-defined dictionary.