import Generator from './generator.js';

const generator = new Generator();

const nicknames = [
  'Apple', 'Boss', 'Crazy', 'Dancer', 'Eagle', 'Flyer', 'Game', 'Hunter', 'Insider', 'Joker',
  'King', 'Lancer', 'Master', 'Ninja', 'Ocean', 'Phoenix', 'Queen', 'Ranger', 'Savage', 'Tornado',
  'Ultra', 'Viking', 'Warrior', 'Xenon', 'Yoda', 'Zodiac', 'Ace', 'Bold', 'Clever', 'Dreams',
  'Epic', 'Frost', 'Gigant', 'Hero', 'Iceberg', 'Jester', 'Knight', 'Lethal', 'Meteor', 'Nova',
  'Orange', 'Pioneer', 'Quasar', 'Rampage', 'Solar', 'Titan', 'Unstoppable', 'Vandal', 'Wizard', 'Xray',
  'Yellow', 'Zeus', 'Awesome', 'Brilliant', 'Courage', 'Dynamite', 'Elegant', 'Fearless', 'Glory', 'Hysteria',
  'Alpha', 'Bionic', 'Cosmic', 'Dominator', 'Electric', 'Fury', 'Gigabyte', 'Heroic', 'Ice', 'Jolt',
  'Killer', 'Laser', 'Meteorite', 'Nemesis', 'Oceanic', 'Paradox', 'Quake', 'Rampart', 'Specter', 'Thunder',
  'Ultimate', 'Vapor', 'Wizardry', 'Xenonix', 'Yellowstone', 'Zigzag', 'Apex', 'Blitz', 'Catalyst', 'Dynamo',
  'Eon', 'Flux', 'Ghost', 'Hawk', 'Infinity', 'Jolted', 'Kaleidoscope', 'Lumina', 'Maelstrom', 'Nebel',
  'Onyx', 'Pulsar', 'Quixote', 'Renegade', 'Skybolt', 'Tornado', 'Umbra', 'Vagabond', 'Wildfire', 'Xylophone'
];

export function mutateText(options: { text: string, advanced: boolean, data?: any }) {
  let text = options.text;

  text = text.replace(/\[(.+)\]/g, (_, value) => generator.chooseRandomValueFromArray(value.split(';')));

  text = text.replace(/#([nlms])\((\d+)-(\d+)\)/g, (match, type, min, max) => {
    let result = match;

    switch (type) {
      case 'n':
        result = generator.generateRandomNumericString(generator.generateRandomNumberBetween(Number(min), Number(max))); break;
      case 'l':
        result = generator.generateRandomLetterString(generator.generateRandomNumberBetween(Number(min), Number(max))); break;
      case 'm':
        result = generator.generateRandomMultiString(generator.generateRandomNumberBetween(Number(min), Number(max))); break;
      case 's':
        result = generator.generateRandomSpecialString(generator.generateRandomNumberBetween(Number(min), Number(max))); break;
      default: 
        break;
    }

    return result;
  });
  
  text = text.replace(/#w\(([yYnN]):([yYnN])\)/g, (_, useGlitch, useWave) => {
    let word = generator.chooseRandomValueFromArray(nicknames);

    if (String(useGlitch).toLowerCase() === 'y') {
      let glitchWord = '';

      for (const char of word) {
        const randomChance = Math.random();

        if (randomChance >= 0.75) {
          glitchWord += char.toLowerCase()
            .replace(/o/g, '0')
            .replace(/a/g, '4')
            .replace(/z/g, '3')
            .replace(/e/g, '3')
            .replace(/i/g, '1')
            .replace(/l/g, '1')
            .replace(/p/g, '5')
            .replace(/v/g, '8')
            .replace(/b/g, '6');
        } else {
          glitchWord += char;
        }
      }

      word = glitchWord;
    }

    if (String(useWave).toLowerCase() === 'y') {
      let waveWord = '';

      for (const char of word) {
        if (Math.random() > 0.6) {
          waveWord += char.toUpperCase();
        } else {
          waveWord += char;
        }
      }

      word = waveWord;
    }

    return word;
  });

  text = text
    .replace(/#n/g, () => generator.generateRandomNumericString(3))
    .replace(/#l/g, () => generator.generateRandomLetterString(3))
    .replace(/#m/g, () => generator.generateRandomMultiString(3))
    .replace(/#s/g, () => generator.generateRandomSpecialString(3));

  if (options.advanced) {
    text = text.replace(/#p/g, () => generator.chooseRandomValueFromArray(options.data.players));
  }
      
  return text;
}