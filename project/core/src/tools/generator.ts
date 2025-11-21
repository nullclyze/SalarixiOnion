class Generator {
	public generateRandomNumericString(length: number) {
		const chars = '0123456789';

    let result = '';

    for (let i = 0; i < length; i++) {
      const ind = Math.floor(Math.random() * chars.length);
      result += chars[ind];
    }

    return result;
	}

	public generateRandomLetterString(length: number) {
		const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';

    let result = '';

    for (let i = 0; i < length; i++) {
      const ind = Math.floor(Math.random() * chars.length);
      result += chars[ind];
    }
    	
    return result;
	}

	public generateRandomMultiString(length: number) {
		const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

    let result = '';

    for (let i = 0; i < length; i++) {
      const ind = Math.floor(Math.random() * chars.length);
      result += chars[ind];
    }

    return result;
	}

	public generateRandomSpecialString(length: number) {
		const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=';

    let result = '';

    for (let i = 0; i < length; i++) {
      const ind = Math.floor(Math.random() * chars.length);
      result += chars[ind];
    }

    return result;
	}

	public generateRandomNumberBetween(min: number, max: number) {
		return Math.floor(Math.random() * (max - min + 1) + min);
	}

  public generateRandomNumberBetweenFloat(min: number, max: number) {
    return Math.random() * (max - min + 1) + min;
  }

	public chooseRandomValueFromArray(array: any[]) {
		return array[Math.floor(Math.random() * array.length)];
	}
}

export default Generator;